Task 1: Find the top `Combo` per instrument, then find the combination with the highest `point`

Explanation:
`YorexInstrument2` contains trading data. The `combos` field contains an array of `ComboGroup`, and `ComboGroup` consists of 1 or more `Combo`. A `Combo` is a strategy, e.g. Buy when rsi is greater than 80. It contains `points` that represent profit and losses for each `Instrument`.

Use Case 1 - Top Combo:
Finding the top combo means you want to find a `Combo` with the highest `point` for an instrument out of all `Combo`s. Alone this is not helpful information.

Use Case 2 - Top ComboGroup:
It means you want to find a group of `Combo` that has the highest` point` for an `Instrument` out of all `ComboGroup`s. This information is more valuable than Use Case 1.

Use Case 3 - Top Instruments:
It means you want to find the `ComboGroup` with the highest points per `Instrument`. This information tells us the highest performing strategy for an `Instrument`.

Use Case 4 - Combining:
Combining means you take an array of results for example `[1,2,3]` and produce all possible combinations e.g. `[1],[1,2],[1,2,3],[1,3],[2,3],[2],[3]`. Combining sets of `Combo` is useful to create new strategies.
 
