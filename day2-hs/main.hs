module Main where

main :: IO ()
main = do
  part1Result <- part1
  part2Result <- part2
  putStrLn $ "Part 1: " ++ show part1Result
  putStrLn $ "Part 2: " ++ show part2Result

part1 :: IO Int
part1 = do
  content <- readFile "input.txt"
  let reports = readReports content
  return $ length $ filter isValid reports

part2 :: IO Int
part2 = do
  content <- readFile "input.txt"
  let reports = readReports content
  return $ length $ filter isValid2 reports

readReports :: String -> [[Int]]
readReports = map (map read . words) . lines

isValid :: [Int] -> Bool
isValid xs = isValidAsc xs || isValidDesc xs

isValidAsc :: [Int] -> Bool
isValidAsc [] = True
isValidAsc [x] = True
isValidAsc (x1 : x2 : xs) = abs (x2 - x1) <= 3 && x2 > x1 && isValidAsc (x2 : xs)

isValidDesc :: [Int] -> Bool
isValidDesc [] = True
isValidDesc [x] = True
isValidDesc (x1 : x2 : xs) = abs (x2 - x1) <= 3 && x2 < x1 && isValidDesc (x2 : xs)

isValid2 :: [Int] -> Bool
isValid2 xs = isValid xs || any (isValidIgnoringElement xs) [0 .. length xs - 1]

isValidIgnoringElement :: [Int] -> Int -> Bool
isValidIgnoringElement xs i = let (a, _ : b) = splitAt i xs in isValid (a ++ b)
