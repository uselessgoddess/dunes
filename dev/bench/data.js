window.BENCHMARK_DATA = {
  "lastUpdate": 1764770087374,
  "repoUrl": "https://github.com/uselessgoddess/dunes",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b9d638a0a06fdbb1367d01f63e8887c11586c47a",
          "message": "Merge pull request #13 from konard/issue-9-2fa720aa7697\n\nFix git identity configuration for gh-pages branch creation",
          "timestamp": "2025-12-02T18:24:33+03:00",
          "tree_id": "f24763708b848d59d63f0fcbe31f03b7cf504b6f",
          "url": "https://github.com/uselessgoddess/dunes/commit/b9d638a0a06fdbb1367d01f63e8887c11586c47a"
        },
        "date": 1764689241470,
        "tool": "cargo",
        "benches": [
          {
            "name": "sbt_insert_100/usize/100",
            "value": 1833,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_100/nonzero/100",
            "value": 1441,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000/usize/1000",
            "value": 26110,
            "range": "± 2258",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000/nonzero/1000",
            "value": 20146,
            "range": "± 792",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000/usize/10000",
            "value": 325232,
            "range": "± 1592",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000/nonzero/10000",
            "value": 266493,
            "range": "± 1435",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_100/usize/100",
            "value": 2377,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_1000/usize/1000",
            "value": 36594,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_10000/usize/10000",
            "value": 745583,
            "range": "± 2811",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_100/usize/100",
            "value": 1905,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_1000/usize/1000",
            "value": 25673,
            "range": "± 248",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0f4f09670fb1d45469d3ea91624fb42571eb37ad",
          "message": "Merge pull request #11 from konard/issue-10-9a1c12f6ea06\n\nRefactor trees crate: reorganize test utilities and add generic benchmarks",
          "timestamp": "2025-12-03T16:53:50+03:00",
          "tree_id": "aa4c174374fbca3c79aa902c0ec11a3a7de072b2",
          "url": "https://github.com/uselessgoddess/dunes/commit/0f4f09670fb1d45469d3ea91624fb42571eb37ad"
        },
        "date": 1764770087130,
        "tool": "cargo",
        "benches": [
          {
            "name": "sbt_full_cycle_100",
            "value": 1898.42,
            "range": "± 25.00",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_full_cycle_1000",
            "value": 25466.19,
            "range": "± 2169.11",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_100",
            "value": 1662.83,
            "range": "± 98.86",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_1000",
            "value": 24335.06,
            "range": "± 2483.11",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_10000",
            "value": 297367.53,
            "range": "± 4455.89",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_100",
            "value": 2247.28,
            "range": "± 38.63",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_1000",
            "value": 35426.64,
            "range": "± 1796.42",
            "unit": "ns/iter"
          },
          {
            "name": "sbt_insert_search_10000",
            "value": 751530.15,
            "range": "± 7305.01",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}