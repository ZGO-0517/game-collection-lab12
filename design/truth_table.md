| Current State (alive) | Live Neighbors | Next State | Rule Applied / Reason |
|-----------------------|----------------|------------|-----------------------|
| true                  | 0              | false      | Underpopulation (< 2) |
| true                  | 1              | false      | Underpopulation (< 2) |
| true                  | 2              | true       | Survives (2 or 3)     |
| true                  | 3              | true       | Survives (2 or 3)     |
| true                  | 4              | false      | Overpopulation (> 3)  |
| true                  | 5              | false      | Overpopulation (> 3)  |
| true                  | 6              | false      | Overpopulation (> 3)  |
| true                  | 7              | false      | Overpopulation (> 3)  |
| true                  | 8              | false      | Overpopulation (> 3)  |
| false                 | 0              | false      | Stays dead            |
| false                 | 1              | false      | Stays dead            |
| false                 | 2              | false      | Stays dead            |
| false                 | 3              | true       | Reproduction (exactly 3) |
| false                 | 4              | false      | Stays dead            |
| false                 | 5              | false      | Stays dead            |
| false                 | 6              | false      | Stays dead            |
| false                 | 7              | false      | Stays dead            |
| false                 | 8              | false      | Stays dead            |
