import Control.Monad

main = do
  [h,w] <- (map read . words) <$> getLine
  ss <- replicateM h getLine
  putStrLn $ show $ initg h w $ solve ss

solve :: [String] -> [[String]]
solve xs = words <$> xs

initg :: Int -> Int -> [[String]] -> [Int]
initg h w xs = (serchs w) <$> (drop (h - 1)) (concat xs)
  

serchs :: Int -> String -> Int
serchs w (x:xs)
  | w == 0    = 0
  | x == 's'  = w
  | otherwise = serchs (w - 1) xs

mmain :: Int -> [[String]] -> [Int] -> [Int]
mmain time xs place 
  | time  == 0   = [1 :: Int] 
  | place == 2   = (mmain  (time - 1) (take (time - 1) (concat xs))) <$> ((serchs 3) <$> (drop (time - 1) (concat xs)))
  | otherwise    = [3 :: Int]

