module Main where

import System.Environment
import System.IO

-- get test directory
-- walk test directory building list of files
-- for each file
--      print filename
--      if not UNIMPLEMENTED
--          get expectations
--          cargo run -- filename
--          compare output to expectations
--          if same
--              print PASSED
--          else
--              print FAILED and diff
--      else
--          print UNIMPLEMENTED
main :: IO ()
main = do
  args <- getArgs
  let testDir = args !! 0
  putStrLn testDir
  return ()
