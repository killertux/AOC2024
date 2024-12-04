module Main where
import Data.List (sort)
import Data.Map (Map, fromListWith, findWithDefault, (!?))

main :: IO ()
main = do
    part1Result <- part1
    part2Result <- part2
    putStrLn $ "Part 1: " ++ show part1Result
    putStrLn $ "Part 2: " ++ show part2Result

part1 :: IO Int
part1 = do
    content <- readFile "input.txt"
    let pairs = map (toTupleOfInt . words) (lines content)
        (list1, list2) = unzip pairs
        sortedList1 = sort list1
        sortedList2 = sort list2
    return $ sum $ zipWith diff sortedList1 sortedList2

part2 :: IO Int
part2 = do
    content <- readFile "input.txt"
    let pairs = map (toTupleOfInt . words) (lines content)
        (list1, list2) = unzip pairs
        occurencesList2 = occurences list2
    return $ sum $ map (\x -> x * (orElse (occurencesList2 !? x) 0)) list1

toTupleOfInt :: [String] -> (Int, Int)
toTupleOfInt [x, y] = (read x, read y)

diff :: Int -> Int -> Int
diff x y = abs (x - y)

occurences :: [Int] -> Map Int Int
occurences list = fromListWith (+) [(x, 1) | x <- list]

orElse :: Maybe a -> a -> a
orElse (Just x) _ = x
orElse Nothing  y = y
