import System.IO
import Data.Char (isDigit)
import Data.List (find)

main :: IO ()
main = do
  -- Open the input file
  -- handle <- openFile "test-input.txt" ReadMode
  handle <- openFile "input.txt" ReadMode

  -- Read lines from the file and process each line
  result <- processLines handle []

  let finalResult = sum (map read result :: [Int])

  putStrLn $ "Final result: " ++ show finalResult


  -- Close the file
  hClose handle

processLines :: Handle -> [String] -> IO [String]
processLines handle resultList = do
  eof <- hIsEOF handle
  if eof
    then return resultList
    else do
      line <- hGetLine handle
      let firstChar = maybeToList (find isDigit line)
      let secondChar = maybeToList (findFromRight isDigit line)
      let combined = firstChar ++ secondChar
      processLines handle (resultList ++ [combined])

-- Utility functions
findFromRight :: (a -> Bool) -> [a] -> Maybe a
findFromRight predicate lst = find predicate (reverse lst)

maybeToList :: Maybe a -> [a]
maybeToList Nothing = []
maybeToList (Just x) = [x]