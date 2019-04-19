kurol :: Int -> String
kurol 0 _ = "\n"
kurol v xs = xs : kurol u (v - 1) (xs : #) 

kurol :: Int -> String
kuror 0 _ = ""
kuror 0 _ = ""

main = do
  putStrLn 
