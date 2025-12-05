window.BENCHMARK_DATA = {
  "lastUpdate": 1764918966480,
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
          "id": "a55cd396d479587b362f1baf5b95e9c0652e4061",
          "message": "Merge pull request #37 from konard/issue-36-7d1e5a8dd98e\n\nPort old doublets codebase to new environment",
          "timestamp": "2025-12-04T16:34:51+03:00",
          "tree_id": "f75c7acc56cf547212affb5325c6fd06049308f8",
          "url": "https://github.com/uselessgoddess/dunes/commit/a55cd396d479587b362f1baf5b95e9c0652e4061"
        },
        "date": 1764855556614,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.77,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1673.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 41.22,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=24261.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.05,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=312000.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 88.33,
            "range": "± 0.23",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2264.30ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 57.31,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=34898.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.61,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=751710.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115.1,
            "range": "± 0.38",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1737.60ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 83.25,
            "range": "± 0.25",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=24024.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 63.45,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=315220.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 68.48,
            "range": "± 0.18",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1460.20ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.76,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15685.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 63.15,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=158350.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.93,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1563.30ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.57,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16588.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.85,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166880.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.4,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1675.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.16,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17674.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 112.23,
            "range": "± 0.21",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=178210.00ns"
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
          "id": "8d7e2cb1abf50144c901cf4c8729136c2a22c865",
          "message": "Merge pull request #39 from konard/issue-38-873d9385e576\n\nAdd tabs to switch between Trees and Doublets benchmarks",
          "timestamp": "2025-12-04T17:08:42+03:00",
          "tree_id": "9732e309b631fa0a865b9dcda3652eea98eb2ded",
          "url": "https://github.com/uselessgoddess/dunes/commit/8d7e2cb1abf50144c901cf4c8729136c2a22c865"
        },
        "date": 1764857591601,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.74,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=100 time=1673.90ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 41.33,
            "range": "± 0.45",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=1000 time=24198.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.22,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=10000 time=310390.00ns"
          },
          {
            "name": "Search Links",
            "value": 87.23,
            "range": "± 0.77",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=2292.80ns"
          },
          {
            "name": "Search Links",
            "value": 55.64,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=35948.00ns"
          },
          {
            "name": "Search Links",
            "value": 26.43,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=756680.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.69,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=200 time=1743.80ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 80.84,
            "range": "± 0.75",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=2000 time=24741.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 64.03,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=20000 time=312360.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.95,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=100 time=1471.70ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=1000 time=15873.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 63.27,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=10000 time=158050.00ns"
          },
          {
            "name": "Search Links",
            "value": 127.83,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=1564.60ns"
          },
          {
            "name": "Search Links",
            "value": 120.71,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=16568.00ns"
          },
          {
            "name": "Search Links",
            "value": 119.93,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=166770.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 118.71,
            "range": "± 0.86",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=200 time=1684.80ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.81,
            "range": "± 0.45",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=2000 time=17573.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 113.5,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=20000 time=176210.00ns"
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
          "id": "d82563d27cd947e9becb19c63b50d21a2343d0f9",
          "message": "Merge pull request #41 from konard/issue-40-b32f0a529654\n\nFix doublets benchmarks and prevent silent pipeline ignore",
          "timestamp": "2025-12-04T17:48:06+03:00",
          "tree_id": "3a1a84088c4ce2da89007d928c91de3698e73daa",
          "url": "https://github.com/uselessgoddess/dunes/commit/d82563d27cd947e9becb19c63b50d21a2343d0f9"
        },
        "date": 1764860011759,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.06,
            "range": "± 0.21",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=100 time=1693.30ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 42.22,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=1000 time=23683.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.07,
            "range": "± 0.37",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=10000 time=311850.00ns"
          },
          {
            "name": "Search Links",
            "value": 88.81,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=2252.10ns"
          },
          {
            "name": "Search Links",
            "value": 57.16,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=34991.00ns"
          },
          {
            "name": "Search Links",
            "value": 26.65,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=750500.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.02,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=200 time=1754.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 83.34,
            "range": "± 0.29",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=2000 time=23999.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 63.96,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=20000 time=312710.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 68.68,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=100 time=1456.10ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.65,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=1000 time=15710.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.94,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=10000 time=158890.00ns"
          },
          {
            "name": "Search Links",
            "value": 128.11,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=1561.20ns"
          },
          {
            "name": "Search Links",
            "value": 120.63,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=16580.00ns"
          },
          {
            "name": "Search Links",
            "value": 119.68,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=167110.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.25,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=200 time=1677.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.55,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=2000 time=17613.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 113.77,
            "range": "± 0.18",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=20000 time=175790.00ns"
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
          "id": "5e13779ffdd770e0588b6b2d518e9681a2684eee",
          "message": "Merge pull request #45 from konard/issue-44-9a6372a638e0\n\nFix SBT remove bug by matching C# reference implementation",
          "timestamp": "2025-12-05T09:55:06+03:00",
          "tree_id": "eeb72b6a70eadfa9db49a186e87be7076c381b33",
          "url": "https://github.com/uselessgoddess/dunes/commit/5e13779ffdd770e0588b6b2d518e9681a2684eee"
        },
        "date": 1764918052731,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.86,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=100 time=1670.50ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 42.93,
            "range": "± 0.24",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=1000 time=23296.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.43,
            "range": "± 0.25",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=10000 time=308350.00ns"
          },
          {
            "name": "Search Links",
            "value": 88.83,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=2251.40ns"
          },
          {
            "name": "Search Links",
            "value": 57.63,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=34704.00ns"
          },
          {
            "name": "Search Links",
            "value": 26.63,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=751100.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 71.56,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=200 time=2794.90ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 49.61,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=2000 time=40315.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 36.71,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=20000 time=544850.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.06,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=100 time=1491.20ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 64.8,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=1000 time=15433.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 64.21,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=10000 time=155750.00ns"
          },
          {
            "name": "Search Links",
            "value": 128.05,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=1561.90ns"
          },
          {
            "name": "Search Links",
            "value": 121,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=16529.00ns"
          },
          {
            "name": "Search Links",
            "value": 120.11,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=166520.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.55,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=200 time=1673.00ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.17,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=2000 time=17672.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 113.18,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=20000 time=176710.00ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dodickgod@gmail.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "dodickgod@gmail.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "distinct": true,
          "id": "629ac736e31304e915a5b1455602f785e304ccc4",
          "message": "Tiny fmt fixes",
          "timestamp": "2025-12-05T08:00:52+01:00",
          "tree_id": "d7ef84736dac9785610ca81526e06eb48be822e8",
          "url": "https://github.com/uselessgoddess/dunes/commit/629ac736e31304e915a5b1455602f785e304ccc4"
        },
        "date": 1764918431893,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 58.93,
            "range": "± 0.37",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=100 time=1697.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 41.68,
            "range": "± 0.47",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=1000 time=23993.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.41,
            "range": "± 0.26",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=10000 time=308540.00ns"
          },
          {
            "name": "Search Links",
            "value": 89.11,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=2244.50ns"
          },
          {
            "name": "Search Links",
            "value": 56.14,
            "range": "± 0.34",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=35623.00ns"
          },
          {
            "name": "Search Links",
            "value": 26.64,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=750650.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 68.08,
            "range": "± 0.28",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=200 time=2937.90ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 48.23,
            "range": "± 0.31",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=2000 time=41472.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 36.95,
            "range": "± 0.24",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=20000 time=541210.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.11,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=100 time=1490.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 64.74,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=1000 time=15447.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 64.49,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=10000 time=155060.00ns"
          },
          {
            "name": "Search Links",
            "value": 127.59,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=1567.50ns"
          },
          {
            "name": "Search Links",
            "value": 120.34,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=16620.00ns"
          },
          {
            "name": "Search Links",
            "value": 119.87,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=166850.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.65,
            "range": "± 0.1",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=200 time=1671.50ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 112.95,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=2000 time=17707.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 112.36,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=20000 time=178000.00ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dodickgod@gmail.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "dodickgod@gmail.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "distinct": true,
          "id": "71dea2efbf654805bc0d0402dcb14a1eedd44000",
          "message": "fix clippy",
          "timestamp": "2025-12-05T08:10:11+01:00",
          "tree_id": "345a5ad0f01305fc2eddd7509da8ac5bb04b5b09",
          "url": "https://github.com/uselessgoddess/dunes/commit/71dea2efbf654805bc0d0402dcb14a1eedd44000"
        },
        "date": 1764918965902,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 59.03,
            "range": "± 0.23",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=100 time=1694.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 45.01,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=1000 time=22216.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33.4,
            "range": "± 0.28",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=10000 time=299430.00ns"
          },
          {
            "name": "Search Links",
            "value": 87.52,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=2285.30ns"
          },
          {
            "name": "Search Links",
            "value": 58.72,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=34062.00ns"
          },
          {
            "name": "Search Links",
            "value": 26.91,
            "range": "± 0.09",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=743090.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 70.88,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=200 time=2821.70ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 49.87,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=2000 time=40102.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 37.88,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "category=Trees tree=SBT ops=20000 time=527950.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 66.93,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=100 time=1494.10ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 64.75,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=1000 time=15443.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 64.3,
            "range": "± 0.04",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=10000 time=155510.00ns"
          },
          {
            "name": "Search Links",
            "value": 126.73,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=200 time=1578.10ns"
          },
          {
            "name": "Search Links",
            "value": 120.59,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=2000 time=16585.00ns"
          },
          {
            "name": "Search Links",
            "value": 119.83,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "category=Doublets tree=N/A ops=20000 time=166910.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.53,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=200 time=1673.20ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 112.86,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=2000 time=17721.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 112.32,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "category=Trees tree=ART ops=20000 time=178070.00ns"
          }
        ]
      }
    ]
  }
}