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

(defn gen-sum [pots, gen]
  (reduce +
    (map-indexed
      (fn [idx itm] (if (= itm \#) (- idx (* gen 2)) 0))
      pots)))

(defn evolve [init, max]
  (loop [gen 0 pots init previous-sum 0]
    (def current-sum (gen-sum pots gen))
    (println (str gen ": " current-sum " " (- current-sum previous-sum)))
    (if (< gen max)
      (recur (inc gen) (next-gen pots) current-sum))))

(evolve pots 200)
; On generation 99 the difference converges to +25 per generation
; Value at genreation 98: 3441
; 3441 + (50000000000 - 98) * 25 = 1,250,000,000,991
