module Main where

import Text.Regex.TDFA ((=~))

main :: IO ()
main = do
  part1Result <- part1
  part2Result <- part2
  putStrLn $ "Part 1: " ++ show part1Result
  putStrLn $ "Part 2: " ++ show part2Result

part1 :: IO Int
part1 = do
  content <- readFile "input.txt"
  return $ sum $ map product $ readIntegers content

part2 :: IO Int
part2 = do
  content <- readFile "input.txt"
  return $ sum $ map product $ readIntegersRemovingDonts content

readIntegers :: String -> [[Int]]
readIntegers content = map (tail . map read) (content =~ "mul\\(([[:digit:]]+),([[:digit:]]+)\\)" :: [[String]])

readIntegersRemovingDonts :: String -> [[Int]]
readIntegersRemovingDonts content = internalAcceptIntegers (content =~ "mul\\(([[:digit:]]+),([[:digit:]]+)\\)|do\\(\\)|don't\\(\\)" :: [[String]])

internalAcceptIntegers :: [[String]] -> [[Int]]
internalAcceptIntegers (['d' : 'o' : 'n' : _, _, _] : xs) = consumeUntilDo xs
internalAcceptIntegers (['d' : 'o' : _, _, _] : xs) = internalAcceptIntegers xs
internalAcceptIntegers ((_ : submatches) : xs) = map read submatches : internalAcceptIntegers xs
internalAcceptIntegers _ = []

consumeUntilDo :: [[String]] -> [[Int]]
consumeUntilDo ((('d' : 'o' : '(' : _) : _) : xs) = internalAcceptIntegers xs
consumeUntilDo (_ : xs) = consumeUntilDo xs
consumeUntilDo _ = []
