window.BENCHMARK_DATA = {
  "lastUpdate": 1764834507505,
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
          "id": "1c8287987fe3f62e068a8f14ccd8ceebe724c295",
          "message": "Merge pull request #23 from konard/issue-22-2c1825bb9843\n\nAdd a complete implementation of ART (Adaptive Radix Tree)",
          "timestamp": "2025-12-03T23:33:42+03:00",
          "tree_id": "eb92beb5ddfb9ce4dbe54d689197550547371eb0",
          "url": "https://github.com/uselessgoddess/dunes/commit/1c8287987fe3f62e068a8f14ccd8ceebe724c295"
        },
        "date": 1764794852469,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.75,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "100 operations in 1673.60 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 43.78,
            "range": "± 0.25",
            "unit": "M links/sec",
            "extra": "1000 operations in 22842.00 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.09,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "10000 operations in 311610.00 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 85.2,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "200 operations in 2347.50 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 58.41,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "2000 operations in 34240.00 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.38,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "20000 operations in 758170.00 ns/iter"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 112.32,
            "range": "± 0.39",
            "unit": "M links/sec",
            "extra": "200 operations in 1780.60 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 86.34,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "2000 operations in 23163.00 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 68.05,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "100 operations in 1469.40 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.86,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "1000 operations in 15659.00 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 63.22,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "10000 operations in 158190.00 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.59,
            "range": "± 0.46",
            "unit": "M links/sec",
            "extra": "200 operations in 1567.50 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 121.04,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "2000 operations in 16524.00 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.88,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "20000 operations in 166830.00 ns/iter"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.75,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "200 operations in 1670.20 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 112.84,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "2000 operations in 17724.00 ns/iter"
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
          "id": "133af21e3dbde116920531ef8c416a90a9ce9b06",
          "message": "Merge pull request #29 from konard/issue-28-397ada389b8b\n\nAdd custom benchmark viewer with tree comparison and unit toggles",
          "timestamp": "2025-12-04T10:44:10+03:00",
          "tree_id": "bde13027dd441c672a9926f9bc6c85428c18cc56",
          "url": "https://github.com/uselessgoddess/dunes/commit/133af21e3dbde116920531ef8c416a90a9ce9b06"
        },
        "date": 1764834506916,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.87,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1670.30ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 42.47,
            "range": "± 0.39",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=23546.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 31.66,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=315870.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 88.6,
            "range": "± 0.1",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2257.40ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 56.93,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=35131.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.52,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=754150.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115.71,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1728.40ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 82.19,
            "range": "± 0.73",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=24334.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 63.07,
            "range": "± 0.22",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=317130.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 68.43,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1461.30ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.52,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15742.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.69,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=159520.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.92,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1563.50ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.61,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16582.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.95,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166740.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.59,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1672.40ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.91,
            "range": "± 0.22",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17558.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 111.94,
            "range": "± 0.68",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=178670.00ns"
          }
        ]
      }
    ]
  }
}