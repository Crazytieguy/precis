Score(3000)=0.715 I=0.872 C=0.586 ns_rows≤3K=22/44 (reached=13 partial=3 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 81 | 31 | README headline in README.md |  |  | 1.000 |
| ns | 86 |  | 36 | README title + one-line description | 1.2 |  | 0.958 |
| walker |  | 100 | 19 | go package doc lede in main.go |  |  | 0.965 |
| ns | 112 |  | 26 | main.go package doc line | 1.3 |  | 0.921 |
| walker |  | 122 | 22 | go module identity in go.mod |  |  | 0.934 |
| ns | 139 |  | 27 | go.mod module + Go version | 1.4 |  | 0.897 |
| ns | 219 |  | 80 | go.mod direct dependencies | 1.5 |  | 0.784 |
| walker |  | 254 | 132 | headings outline in README.md |  |  | 0.793 |
| walker |  | 292 | 38 | README.md section #1 |  |  | 0.793 |
| ns | 296 |  | 77 | README -install transcript | 1.6 |  | 0.697 |
| walker |  | 383 | 91 | go module file go.mod |  |  | 0.812 |
| walker |  | 403 | 20 | README.md section #11 |  |  | 0.812 |
| walker |  | 424 | 21 | README.md section #14 |  |  | 0.812 |
| ns | 441 |  | 145 | README cert generation transcript | 1.7 |  | 0.667 |
| ns | 606 |  | 165 | main.go imports | 1.8 |  | 0.526 |
| walker |  | 710 | 286 | go decl names surface in main.go |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:85 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:87 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:171 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:240 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:267 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:307 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:336 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:345 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:358 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:364 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:370 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:375 |  |  | 0.538 |
| walker |  | 710 | 0 | go decl at main.go:382 |  |  | 0.538 |
| walker |  | 729 | 19 | go decl body at main.go:370 |  |  | 0.539 |
| walker |  | 749 | 20 | go decl body at main.go:375 |  |  | 0.539 |
| ns | 774 |  | 168 | main.go top-level fn signatures | 2.1 |  | 0.601 |
| walker |  | 782 | 33 | go decl body at main.go:358 |  |  | 0.601 |
| walker |  | 824 | 42 | go decl body at main.go:364 |  |  | 0.601 |
| walker |  | 887 | 63 | go decl doc at main.go:85 |  |  | 0.601 |
| walker |  | 935 | 48 | go decl body at main.go:336 |  |  | 0.602 |
| ns | 953 |  | 179 | cert.go top-level fn signatures | 2.2 |  | 0.559 |
| walker |  | 1099 | 164 | go decl at main.go:155 |  |  | 0.570 |
| ns | 1171 |  | 218 | mkcert struct + rootName/rootKeyName consts | 2.3 |  | 0.616 |
| ns | 1203 |  | 32 | truststore_darwin.go fn signatures | 2.4 |  | 0.609 |
| walker |  | 1245 | 146 | go decl names surface in truststore_java.go |  |  | 0.612 |
| walker |  | 1245 | 0 | go decl at truststore_java.go:31 |  |  | 0.612 |
| walker |  | 1245 | 0 | go decl at truststore_java.go:57 |  |  | 0.612 |
| walker |  | 1245 | 0 | go decl at truststore_java.go:81 |  |  | 0.612 |
| walker |  | 1245 | 0 | go decl at truststore_java.go:94 |  |  | 0.612 |
| walker |  | 1245 | 0 | go decl at truststore_java.go:110 |  |  | 0.612 |
| walker |  | 1254 | 9 | go decl at truststore_java.go:21 |  |  | 0.612 |
| ns | 1260 |  | 57 | truststore_linux.go fn signatures | 2.5 |  | 0.599 |
| walker |  | 1295 | 41 | go decl doc at truststore_java.go:110 |  |  | 0.599 |
| ns | 1381 |  | 121 | truststore_windows.go fn signatures + windowsRootStore type | 2.6 |  | 0.578 |
| ns | 1454 |  | 73 | truststore_java.go fn signatures | 2.7 |  | 0.591 |
| walker |  | 1468 | 173 | go decl names surface in truststore_nss.go |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:39 |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:73 |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:89 |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:106 |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:120 |  |  | 0.594 |
| walker |  | 1468 | 0 | go decl at truststore_nss.go:131 |  |  | 0.594 |
| walker |  | 1509 | 41 | go decl doc at truststore_nss.go:120 |  |  | 0.594 |
| ns | 1557 |  | 103 | truststore_nss.go fn signatures | 2.8 |  | 0.608 |
| ns | 1672 |  | 115 | README section headings (table of contents) | 2.9 |  | 0.626 |
| walker |  | 1679 | 170 | go decl at main.go:30 |  |  | 0.630 |
| walker |  | 1770 | 91 | go decl body at main.go:345 |  |  | 0.633 |
| ns | 1907 |  | 235 | main.go flag definitions | 3.1 |  | 0.599 |
| walker |  | 1981 | 211 | go decl names surface in cert.go |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:37 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:50 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:148 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:166 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:176 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:202 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:209 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:282 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:310 |  |  | 0.648 |
| walker |  | 1981 | 0 | go decl at cert.go:366 |  |  | 0.648 |
| walker |  | 1998 | 17 | go decl doc at cert.go:282 |  |  | 0.648 |
| walker |  | 2015 | 17 | go decl body at cert.go:366 |  |  | 0.648 |
| walker |  | 2182 | 167 | go package + imports in main.go |  |  | 0.760 |
| walker |  | 2254 | 72 | go package + imports in truststore_nss.go |  |  | 0.760 |
| ns | 2273 |  | 366 | advancedUsage string — per-flag doc | 3.2 |  | 0.681 |
| ns | 2534 |  | 261 | getCAROOT body — CAROOT discovery rules | 3.3 | 2.1 | 0.632 |
| ns | 2632 |  | 98 | storeEnabled body — TRUST_STORES env var | 3.4 | 2.1 | 0.643 |
| walker |  | 2648 | 394 | README.md section #0 |  |  | 0.713 |
| walker |  | 2714 | 66 | go decl body at cert.go:202 |  |  | 0.713 |
| walker |  | 2753 | 39 | README.md section #10 |  |  | 0.713 |
| walker |  | 2793 | 40 | README.md section #9 |  |  | 0.713 |
| walker |  | 2834 | 41 | README.md section #8 |  |  | 0.713 |
| ns | 2846 |  | 214 | shortUsage string | 3.5 |  | 0.715 |
| walker |  | 3143 | 309 | go decl at main.go:49 |  |  | 0.782 |
| ns | 3277 |  | 431 | Run() dispatch — load + warn loop | 3.6 | 2.1 | 0.725 |
| walker |  | 3307 | 164 | go decl body at main.go:382 |  |  | 0.727 |
| walker |  | 3421 | 114 | go package + imports in truststore_java.go |  |  | 0.727 |
| walker |  | 3469 | 48 | README.md section #4 |  |  | 0.727 |
| walker |  | 3549 | 80 | go decl body at cert.go:166 |  |  | 0.728 |
| walker |  | 3589 | 40 | README.md section #22 |  |  | 0.728 |
| walker |  | 3630 | 41 | README.md section #15 |  |  | 0.732 |
| walker |  | 3673 | 43 | README.md section #17 |  |  | 0.732 |
| ns | 3681 |  | 404 | Run() arg validation — hostname / IP / email / URI / punycode | 3.7 | 2.1 | 0.686 |
| walker |  | 3736 | 63 | README.md section #7 |  |  | 0.686 |
| walker |  | 3781 | 45 | README.md section #24 |  |  | 0.686 |
| walker |  | 3895 | 114 | go decl body at truststore_nss.go:120 |  |  | 0.686 |
| walker |  | 4010 | 115 | go decl body at cert.go:37 |  |  | 0.686 |
| walker |  | 4220 | 210 | go decl at truststore_nss.go:17 |  |  | 0.689 |
| ns | 4254 |  | 573 | main() dispatch — flag combinations + Run call | 3.8 | 2.1 | 0.638 |
| walker |  | 4267 | 47 | README.md section #19 |  |  | 0.638 |
| ns | 4322 |  | 68 | truststore_darwin.go package vars | 3.9 |  | 0.633 |
| ns | 4433 |  | 111 | truststore_linux.go package vars | 3.10 |  | 0.624 |
| ns | 4507 |  | 74 | truststore_windows.go FirefoxProfiles + cert-help vars | 3.11 |  | 0.620 |
| walker |  | 4516 | 249 | go decl body at main.go:240 |  |  | 0.669 |
| walker |  | 4565 | 49 | README.md section #21 |  |  | 0.669 |
| walker |  | 4636 | 71 | README.md section #5 |  |  | 0.669 |
| ns | 4681 |  | 174 | truststore_windows.go DLL var block | 3.12 |  | 0.660 |
| walker |  | 4830 | 194 | README.md section #12 |  |  | 0.660 |
| walker |  | 4957 | 127 | go decl body at truststore_java.go:81 |  |  | 0.660 |
| ns | 5049 |  | 368 | truststore_linux.go init() — distro autodetect | 3.13 | 2.5 | 0.638 |
| walker |  | 5200 | 243 | go package + imports in cert.go |  |  | 0.642 |
| walker |  | 5247 | 47 | README.md section #16 |  |  | 0.642 |
| walker |  | 5395 | 148 | go decl body at truststore_java.go:110 |  |  | 0.642 |
| ns | 5430 |  | 381 | truststore_java.go package vars + init | 3.14 | 2.7 | 0.615 |
| walker |  | 5531 | 136 | go decl body at truststore_java.go:94 |  |  | 0.615 |
| walker |  | 5669 | 138 | go decl names surface in truststore_darwin.go |  |  | 0.626 |
| walker |  | 5669 | 0 | go decl at truststore_darwin.go:25 |  |  | 0.626 |
| walker |  | 5669 | 0 | go decl at truststore_darwin.go:52 |  |  | 0.626 |
| walker |  | 5669 | 0 | go decl at truststore_darwin.go:105 |  |  | 0.626 |
| walker |  | 5678 | 9 | go decl at truststore_darwin.go:18 |  |  | 0.628 |
| ns | 5698 |  | 268 | truststore_nss.go package vars | 3.15 |  | 0.642 |
| walker |  | 5702 | 24 | go decl doc at truststore_darwin.go:25 |  |  | 0.642 |
| walker |  | 5843 | 141 | go decl body at truststore_nss.go:73 |  |  | 0.642 |
| walker |  | 5987 | 144 | go decl names surface in truststore_linux.go |  |  | 0.658 |
| walker |  | 5987 | 0 | go decl at truststore_linux.go:27 |  |  | 0.658 |
| walker |  | 5987 | 0 | go decl at truststore_linux.go:51 |  |  | 0.658 |
| walker |  | 5987 | 0 | go decl at truststore_linux.go:55 |  |  | 0.658 |
| walker |  | 5987 | 0 | go decl at truststore_linux.go:77 |  |  | 0.658 |
| walker |  | 6016 | 29 | go decl at truststore_linux.go:17 |  |  | 0.664 |
| walker |  | 6039 | 23 | go decl body at truststore_linux.go:51 |  |  | 0.664 |
| ns | 6085 |  | 387 | truststore_nss.go init() — certutil discovery | 3.16 | 2.8 | 0.638 |
| walker |  | 6186 | 147 | go decl body at truststore_nss.go:106 |  |  | 0.638 |
| walker |  | 6286 | 100 | README.md section #3 |  |  | 0.638 |
| walker |  | 6390 | 104 | README.md section #6 |  |  | 0.638 |
| ns | 6468 |  | 383 | cert.go imports + userAndHostname init() | 4.1 | 2.2 | 0.658 |
| ns | 6698 |  | 230 | makeCert template literal — validity + Subject + KeyUsage | 4.2 | 2.2 | 0.647 |
| walker |  | 6778 | 388 | go decl body at main.go:307 |  |  | 0.649 |
| walker |  | 6785 | 7 | listing of '.github' |  |  | 0.649 |
| walker |  | 6793 | 8 | listing of '.github/workflows' |  |  | 0.649 |
| walker |  | 7003 | 210 | go decl body at cert.go:148 |  |  | 0.649 |
| ns | 7045 |  | 347 | makeCert classification + ExtKeyUsage loop | 4.3 | 2.2 | 0.635 |
| walker |  | 7095 | 92 | README.md section #18 |  |  | 0.635 |
| walker |  | 7310 | 215 | go decl body at truststore_nss.go:131 |  |  | 0.635 |
| ns | 7326 |  | 281 | newCA template literal — root CA defaults | 4.4 | 2.2 | 0.622 |
| walker |  | 7404 | 94 | README.md section #23 |  |  | 0.622 |
| walker |  | 7520 | 116 | README.md section #2 |  |  | 0.622 |
| walker |  | 7590 | 70 | go package + imports in truststore_linux.go |  |  | 0.622 |
| ns | 7594 |  | 268 | fileNames body — output path derivation | 4.5 | 2.2 | 0.607 |
| walker |  | 7841 | 251 | go decl body at cert.go:176 |  |  | 0.633 |
| ns | 7990 |  | 396 | loadCA body — CA load + keyless mode | 4.6 | 2.2 | 0.618 |
| walker |  | 8115 | 274 | go decl body at truststore_java.go:31 |  |  | 0.649 |
| ns | 8174 |  | 184 | generateKey + randomSerialNumber + caUniqueName | 4.7 | 2.2 | 0.653 |
| walker |  | 8193 | 78 | go package + imports in truststore_darwin.go |  |  | 0.653 |
| walker |  | 8284 | 91 | README.md section #20 |  |  | 0.653 |
| walker |  | 8538 | 254 | go decl body at truststore_nss.go:89 |  |  | 0.653 |
| ns | 8554 |  | 380 | makeCertFromCSR template + CSR-omits defaults | 4.8 | 2.2 | 0.639 |
| walker |  | 9099 | 561 | go decl body at main.go:267 |  |  | 0.642 |
| ns | 9122 |  | 568 | install() body — orchestrate all backends | 5.1 | 2.1 | 0.656 |
| walker |  | 9194 | 95 | go package + imports in truststore_windows.go |  |  | 0.656 |
| ns | 9517 |  | 395 | uninstall() body | 5.2 | 2.1 | 0.665 |
| walker |  | 9559 | 365 | go decl names surface in truststore_windows.go |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:35 |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:54 |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:71 |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:83 |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:91 |  |  | 0.686 |
| walker |  | 9559 | 0 | go decl at truststore_windows.go:107 |  |  | 0.686 |
| walker |  | 9568 | 9 | go decl at truststore_windows.go:19 |  |  | 0.687 |
| walker |  | 9577 | 9 | go decl at truststore_windows.go:25 |  |  | 0.688 |
| walker |  | 9640 | 63 | go decl body at truststore_windows.go:83 |  |  | 0.688 |
| ns | 9926 |  | 409 | checkPlatform body + helpers (pathExists/binaryExists/commandWithSudo) | 5.3 | 2.1 | 0.691 |
| walker |  | 9965 | 325 | go decl body at truststore_java.go:57 |  |  | 0.691 |
