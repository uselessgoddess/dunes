window.BENCHMARK_DATA = {
  "lastUpdate": 1764840906245,
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
          "id": "f93fbef8477f83b09309159f35e29e6571beb0f1",
          "message": "Merge pull request #35 from konard/issue-34-46359bba9a08\n\nFix Chart.js date adapter error on GitHub Pages",
          "timestamp": "2025-12-04T12:30:44+03:00",
          "tree_id": "3b29dde7a33518a12d1120559e69af5c7cc0e3fd",
          "url": "https://github.com/uselessgoddess/dunes/commit/f93fbef8477f83b09309159f35e29e6571beb0f1"
        },
        "date": 1764840905851,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 58.97,
            "range": "± 0.5",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1695.80ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 44.38,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=22531.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.37,
            "range": "± 0.49",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=308910.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 87.99,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2273.10ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 58.84,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=33988.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.69,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=749330.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.35,
            "range": "± 1.19",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1749.00ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 87.65,
            "range": "± 0.1",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=22817.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 64.89,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=308220.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.36,
            "range": "± 0.33",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1484.50ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.95,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15637.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 63.14,
            "range": "± 0.23",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=158380.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.73,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1565.80ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.62,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16581.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.94,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166750.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.08,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1679.50ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.15,
            "range": "± 0.24",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17675.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 111.58,
            "range": "± 0.82",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=179240.00ns"
          }
        ]
      }
    ]
  }
}