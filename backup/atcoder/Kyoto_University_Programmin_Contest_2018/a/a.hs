import Control.Monad

main = do
  n <- readLn
  ss <- replicateM 2 getLine
  putStrLn $ show $ cal $ solve1 $ solve n ss

solve :: Int -> [String] -> [[String]]
solve x xs = words <$> xs

solve1 :: [[String]] -> [[Int]]
solve1 xs = map ri xs

ri :: [String] -> [Int]
ri xs = (map read) xs

cal :: [[Int]] -> Int
cal sa = maximum [(s * a) | (s, a) <- zip (head sa) (last sa)]
