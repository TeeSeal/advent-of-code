(use '[clojure.string :only (split split-lines join)])

(def lines (split-lines (slurp "input.txt")))
(def pots (first lines))
(def patterns
  (into {}
    (map (fn [x] (split x #" => ")) (rest lines))))

(defn pattern-at [i, s]
  (join
    (take 5
      (drop (+ i 2) (str "...." s "....")))))

(defn next-gen [x]
  (join
    (map-indexed
      (fn [idx _]
        (get patterns
          (pattern-at idx (str ".." x "..")) "."))
          (str ".." x ".."))))

(def final
  (nth (iterate next-gen pots) 20))

(println
  (reduce +
    (map-indexed (fn [idx itm] (if (= itm \#) (- idx 40) 0)) final)))
