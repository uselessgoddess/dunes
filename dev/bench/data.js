window.BENCHMARK_DATA = {
  "lastUpdate": 1764841720852,
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
          "id": "a64d46aed3074e33ad0d7cc48c1a61f403f2a027",
          "message": "Merge pull request #31 from konard/issue-30-c40765cb9a35\n\nFix empty benchmark charts on github pages",
          "timestamp": "2025-12-04T11:15:06+03:00",
          "tree_id": "efb7e9c37993117a57890cc7eb9611b4f7efe8c8",
          "url": "https://github.com/uselessgoddess/dunes/commit/a64d46aed3074e33ad0d7cc48c1a61f403f2a027"
        },
        "date": 1764836364012,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.74,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1674.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 40.25,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=24844.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 31.94,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=313050.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 89,
            "range": "± 0.1",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2247.30ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 56.04,
            "range": "± 0.25",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=35690.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.41,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=757300.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.08,
            "range": "± 0.18",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1753.20ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 80.6,
            "range": "± 0.6",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=24813.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 63.47,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=315090.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.86,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1473.70ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.45,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15760.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.32,
            "range": "± 1.09",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=160470.00ns"
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
            "value": 120.67,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16574.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.96,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166720.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.3,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1676.40ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.06,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17689.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 113.05,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=176910.00ns"
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
          "id": "4b6cb8615d88456cf449f1d7eaaa1ec8d5d2428b",
          "message": "Merge pull request #33 from konard/issue-32-a993175deace\n\nFix empty charts in GitHub Pages by handling old benchmark format",
          "timestamp": "2025-12-04T11:42:50+03:00",
          "tree_id": "696d1a01b827801e390a45a2b627a2361a1bc4dc",
          "url": "https://github.com/uselessgoddess/dunes/commit/4b6cb8615d88456cf449f1d7eaaa1ec8d5d2428b"
        },
        "date": 1764838031203,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.72,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1674.40ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 40.45,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=24724.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 31.68,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=315630.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 88.87,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2250.40ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 55.82,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=35830.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.45,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=756230.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.8,
            "range": "± 0.36",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1742.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 79.17,
            "range": "± 0.29",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=25261.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 62.12,
            "range": "± 0.46",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=321970.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.14,
            "range": "± 0.75",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1489.40ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 62.84,
            "range": "± 0.38",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15914.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.86,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=159090.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.1,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1573.60ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.71,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16568.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.75,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=167020.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 118.96,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1681.20ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.09,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17685.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 113.38,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=176400.00ns"
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
          "id": "f93fbef8477f83b09309159f35e29e6571beb0f1",
          "message": "Merge pull request #35 from konard/issue-34-46359bba9a08\n\nFix Chart.js date adapter error on GitHub Pages",
          "timestamp": "2025-12-04T12:30:44+03:00",
          "tree_id": "3b29dde7a33518a12d1120559e69af5c7cc0e3fd",
          "url": "https://github.com/uselessgoddess/dunes/commit/f93fbef8477f83b09309159f35e29e6571beb0f1"
        },
        "date": 1764841720534,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 69.11,
            "range": "± 0.27",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1447.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 46.32,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=21590.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33.56,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=297970.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 96.7,
            "range": "± 0.42",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2068.20ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 42.71,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=46826.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.48,
            "range": "± 0.01",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=755390.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 132.53,
            "range": "± 0.66",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1509.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 91.44,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=21872.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 66.8,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=299420.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 72.79,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1373.90ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 61.24,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=16328.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 57.52,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=173840.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 133.88,
            "range": "± 0.28",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1493.90ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.19,
            "range": "± 0.23",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16641.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 112.33,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=178050.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 130.34,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1534.40ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 109.45,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=18273.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 103.25,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=193700.00ns"
          }
        ]
      }
    ]
  }
}