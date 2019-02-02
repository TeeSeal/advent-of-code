import Data.Char
import Data.List
import Data.Map (Map)
import qualified Data.Map as Map

-- input parsing
parseInput = map (drop 1 . filter isUpper) . lines
newKeyOrAppend pair = Map.insertWith (++) (head pair) (tail pair)
buildDataMap = foldr (newKeyOrAppend) Map.empty . parseInput

-- helpers
derive x m = Map.findWithDefault "" x m
deriveAll steps m = foldl (++) "" $ map (\x -> derive x m) steps
isReq x (_, values) = x `elem` values
reqs step list = map (fst) $ filter (isReq step) list
reqsDone step done list = all (`elem` done) $ reqs step list

asTime x = (x, (ord x) - 4)
decTime (x, time) = (x, (pred time))
positiveTime (_, time) = time > 0
zeroTime (_, time) = time <= 0

-- part 1
part1 [] out _ = reverse out
part1 (head:tail) out m = part1 (sort (tail ++ filtered)) output m
  where
    output = (head:out)
    derived = derive head m
    list = Map.toList m
    filtered = filter (\x -> reqsDone x output list) derived


-- part 2
part2 [] _ [] total _ = total
part2 todo done times total m = part2 newTodo newDone newTimes (total + 1) m
  where
    count = 5 - length times
    cando = filter (\x -> reqsDone x done (Map.toList m)) todo
    taken = take count cando

    times2 = times ++ map (asTime) taken
    decdTimes = map (decTime) times2
    completedTimes = filter (zeroTime) decdTimes
    newTimes = decdTimes \\ completedTimes

    newDone = done ++ justCompleted

    inProgress = map (fst) newTimes
    justCompleted = map (fst) completedTimes
    addedTodos = filter (\x -> reqsDone x newDone (Map.toList m)) $ (deriveAll justCompleted m) \\ inProgress
    newTodo = sort ((todo \\ taken) ++ addedTodos)



main = do
  input <- readFile "input.txt"
  let dataMap = buildDataMap input
  let dataList = Map.toList $ dataMap

  let isEntryPoint x = not $ any (elem x) $ map (snd) dataList
  let entryPoints = sort $ filter (isEntryPoint) $ map (fst) dataList

  putStrLn $ show $ part1 entryPoints "" dataMap
  putStrLn $ show $ part2 entryPoints "" [] 0 dataMap
