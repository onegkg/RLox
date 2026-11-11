#!/usr/bin/env runghc
module Main where

import Control.Monad (filterM, foldM, forM)
import Data.Foldable (for_)
import Data.List (isInfixOf, isPrefixOf, (!?))
import Data.Maybe (fromMaybe, mapMaybe)
import Distribution.Utils.Generic (fstOf3)
import GHC.IO.Exception (ExitCode (ExitFailure, ExitSuccess))
import System.Directory (doesDirectoryExist, listDirectory)
import System.Environment
import System.FilePath ((</>))
import System.IO
import System.Process (readProcessWithExitCode)
import Text.Printf

data Expectation
  = Output String -- "expect: "
  | RuntimeError String -- "expect runtime error: "
  | ParseError String -- "Error at "

data Result
  = Failed
  | Passed
  | Unimplemented

splitOn :: (Eq a) => [a] -> [a] -> [[a]]
splitOn [] _ = [[]]
splitOn sub str = go str
  where
    n = length sub
    go [] = [[]]
    go s@(c : cs)
      | sub `isPrefixOf` s = [] : go (drop n s)
      | otherwise = let (x : xs) = go cs in (c : x) : xs

buildTestList :: FilePath -> IO [FilePath]
buildTestList fp = do
  items <- listDirectory fp
  let paths = map (fp </>) items

  subdirs <- filterM doesDirectoryExist paths
  files <- filterM (fmap not . doesDirectoryExist) paths

  recursiveFiles <- fmap (concat) (mapM buildTestList subdirs)
  return (files ++ recursiveFiles)

processFile :: FilePath -> IO Result
processFile fp = do
  contents <- readFile fp
  let fileLines = lines contents
  let first = fromMaybe "" (fileLines !? 0)
  if "UNIMPLEMENTED" `isInfixOf` first
    then
      return Unimplemented
    else
      printf "\t- %s -> " fp
        >> processFileInner fp

processFileInner :: FilePath -> IO Result
processFileInner fp =
  do
    -- [x] parse expectations
    f <- (readFile fp)
    let fl = lines f
    let expectations = parseExpectations fl
    (exit, stdout, stderr) <- readProcessWithExitCode "./target/debug/crafting_interpreters_rs" [fp] ""
    let (expectationsMet, msgs) = checkExpectations expectations stdout stderr
    let (ok, expected) = checkExitCode expectations exit
    ( if ok
        then
          if expectationsMet
            then
              putStrLn "PASSED" >> return Passed
            else
              putStr "FAILED: " >> for_ msgs putStrLn >> return Failed
        else
          printf "FAILED: Incorrect exit code, expected %s; got %s\n" (show expected) (show exit) >> return Failed
      )

checkExitCode :: [Expectation] -> ExitCode -> (Bool, ExitCode)
checkExitCode es exit =
  let expected =
        foldl
          ( \acc -> \e -> case e of
              Output _ -> ExitSuccess
              RuntimeError _ -> ExitFailure 70
              ParseError _ -> ExitFailure 65
          )
          ExitSuccess
          es
   in (exit == expected, expected)

parseExpectations :: [String] -> [Expectation]
parseExpectations ss = mapMaybe parseExpectationFromLine ss

checkExpectations :: [Expectation] -> String -> String -> (Bool, [String])
checkExpectations expectations stdout stderr =
  foldl
    ( \acc -> \e ->
        let (passed, msg) =
              ( case e of
                  Output o -> checkOutput o stdout
                  RuntimeError r -> checkRuntimeError r stderr
                  ParseError p -> checkParseError p stderr
              )
         in (fst acc && passed, if msg /= "" then msg : snd acc else snd acc)
    )
    (True, [])
    expectations

checkOutput :: String -> String -> (Bool, String)
checkOutput o stdout = if o `isInfixOf` stdout then (True, "") else (False, printf "Expected stdout to contain %s; got %s" o stdout)

checkRuntimeError :: String -> String -> (Bool, String)
checkRuntimeError e stderr = if e `isInfixOf` stderr then (True, "") else (False, printf "Expected stderr to contain %s; got %s" e stderr)

checkParseError :: String -> String -> (Bool, String)
checkParseError e stderr = if e `isInfixOf` stderr then (True, "") else (False, printf "Expected stderr to contain \"%s\"; got \"%s\"" e stderr)

parseExpectationFromLine :: String -> Maybe Expectation
parseExpectationFromLine s = case splitOn "expect: " s !? 1 of
  Just val -> Just (Output val)
  Nothing -> case splitOn "expect runtime error: " s !? 1 of
    Just val -> Just (RuntimeError val)
    Nothing -> case splitOn "Error: " s !? 1 of
      Just val -> Just (ParseError val)
      Nothing -> Nothing

main :: IO ()
main = do
  args <- getArgs
  let testDir = args !! 0
  files <- buildTestList testDir
  results <- mapM processFile files
  let (passed, failed, skipped) =
        foldl
          ( \(passed :: Int, failed :: Int, skipped :: Int) -> \e -> case e of
              Unimplemented -> (passed, failed, skipped + 1)
              Passed -> (passed + 1, failed, skipped)
              Failed -> (passed, failed + 1, skipped)
          )
          (0, 0, 0)
          results
  printf "=====TESTS FINISHED=====\n"
  printf "Passed: %d\n" passed
  printf "Failed: %d\n" failed
  printf "Skipped: %d\n" skipped
