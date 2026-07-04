Score(3000)=0.767 I=0.917 C=0.641 ns_rows≤3K=22/44 (reached=16 partial=0 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 86 | 36 | README headline in README.md |  |  | 1.000 |
| ns | 86 |  | 36 | README title + one-line description | 1.2 |  | 1.000 |
| walker |  | 105 | 19 | go package doc lede in main.go |  |  | 1.000 |
| ns | 112 |  | 26 | main.go package doc line | 1.3 |  | 0.959 |
| walker |  | 132 | 27 | go module identity in go.mod |  |  | 0.968 |
| ns | 139 |  | 27 | go.mod module + Go version | 1.4 |  | 0.964 |
| ns | 219 |  | 80 | go.mod direct dependencies | 1.5 |  | 0.843 |
| walker |  | 267 | 135 | headings outline in README.md |  |  | 0.853 |
| ns | 296 |  | 77 | README -install transcript | 1.6 |  | 0.749 |
| walker |  | 308 | 41 | README.md section #1 |  |  | 0.749 |
| walker |  | 409 | 101 | go module file go.mod |  |  | 0.861 |
| walker |  | 429 | 20 | README.md section #11 |  |  | 0.861 |
| ns | 441 |  | 145 | README cert generation transcript | 1.7 |  | 0.708 |
| walker |  | 450 | 21 | README.md section #14 |  |  | 0.708 |
| ns | 606 |  | 165 | main.go imports | 1.8 |  | 0.558 |
| walker |  | 745 | 295 | go decl names surface in main.go |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:85 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:87 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:171 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:240 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:267 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:307 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:336 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:345 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:358 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:364 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:370 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:375 |  |  | 0.571 |
| walker |  | 745 | 0 | go decl at main.go:382 |  |  | 0.571 |
| walker |  | 764 | 19 | go decl body at main.go:370 |  |  | 0.571 |
| ns | 774 |  | 168 | main.go top-level fn signatures | 2.1 |  | 0.628 |
| walker |  | 784 | 20 | go decl body at main.go:375 |  |  | 0.628 |
| walker |  | 817 | 33 | go decl body at main.go:358 |  |  | 0.628 |
| walker |  | 859 | 42 | go decl body at main.go:364 |  |  | 0.629 |
| walker |  | 922 | 63 | go decl doc at main.go:85 |  |  | 0.629 |
| ns | 953 |  | 179 | cert.go top-level fn signatures | 2.2 |  | 0.584 |
| walker |  | 975 | 53 | go decl body at main.go:336 |  |  | 0.585 |
| walker |  | 1149 | 174 | go decl at main.go:155 |  |  | 0.597 |
| ns | 1171 |  | 218 | mkcert struct + rootName/rootKeyName consts | 2.3 |  | 0.651 |
| ns | 1203 |  | 32 | truststore_darwin.go fn signatures | 2.4 |  | 0.644 |
| ns | 1260 |  | 57 | truststore_linux.go fn signatures | 2.5 |  | 0.630 |
| walker |  | 1298 | 149 | go decl names surface in truststore_java.go |  |  | 0.634 |
| walker |  | 1298 | 0 | go decl at truststore_java.go:31 |  |  | 0.634 |
| walker |  | 1298 | 0 | go decl at truststore_java.go:57 |  |  | 0.634 |
| walker |  | 1298 | 0 | go decl at truststore_java.go:81 |  |  | 0.634 |
| walker |  | 1298 | 0 | go decl at truststore_java.go:94 |  |  | 0.634 |
| walker |  | 1298 | 0 | go decl at truststore_java.go:110 |  |  | 0.634 |
| walker |  | 1307 | 9 | go decl at truststore_java.go:21 |  |  | 0.634 |
| walker |  | 1348 | 41 | go decl doc at truststore_java.go:110 |  |  | 0.634 |
| ns | 1381 |  | 121 | truststore_windows.go fn signatures + windowsRootStore type | 2.6 |  | 0.611 |
| ns | 1454 |  | 73 | truststore_java.go fn signatures | 2.7 |  | 0.623 |
| walker |  | 1521 | 173 | go decl names surface in truststore_nss.go |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:39 |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:73 |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:89 |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:106 |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:120 |  |  | 0.626 |
| walker |  | 1521 | 0 | go decl at truststore_nss.go:131 |  |  | 0.626 |
| ns | 1557 |  | 103 | truststore_nss.go fn signatures | 2.8 |  | 0.639 |
| walker |  | 1562 | 41 | go decl doc at truststore_nss.go:120 |  |  | 0.639 |
| walker |  | 1653 | 91 | go decl body at main.go:345 |  |  | 0.642 |
| ns | 1672 |  | 115 | README section headings (table of contents) | 2.9 |  | 0.657 |
| walker |  | 1867 | 214 | go decl names surface in cert.go |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:37 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:50 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:148 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:166 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:176 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:202 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:209 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:282 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:310 |  |  | 0.708 |
| walker |  | 1867 | 0 | go decl at cert.go:366 |  |  | 0.708 |
| walker |  | 1884 | 17 | go decl doc at cert.go:282 |  |  | 0.708 |
| walker |  | 1901 | 17 | go decl body at cert.go:366 |  |  | 0.709 |
| ns | 1907 |  | 235 | main.go flag definitions | 3.1 |  | 0.670 |
| walker |  | 2099 | 198 | go decl at main.go:30 |  |  | 0.676 |
| ns | 2273 |  | 366 | advancedUsage string — per-flag doc | 3.2 |  | 0.606 |
| walker |  | 2276 | 177 | go package + imports in main.go |  |  | 0.711 |
| walker |  | 2353 | 77 | go package + imports in truststore_nss.go |  |  | 0.711 |
| ns | 2534 |  | 261 | getCAROOT body — CAROOT discovery rules | 3.3 | 2.1 | 0.659 |
| ns | 2632 |  | 98 | storeEnabled body — TRUST_STORES env var | 3.4 | 2.1 | 0.670 |
| walker |  | 2777 | 424 | README.md section #0 |  |  | 0.754 |
| walker |  | 2843 | 66 | go decl body at cert.go:202 |  |  | 0.754 |
| ns | 2846 |  | 214 | shortUsage string | 3.5 |  | 0.767 |
| walker |  | 2882 | 39 | README.md section #10 |  |  | 0.767 |
| walker |  | 3046 | 164 | go decl body at main.go:382 |  |  | 0.769 |
| walker |  | 3091 | 45 | README.md section #9 |  |  | 0.769 |
| walker |  | 3137 | 46 | README.md section #8 |  |  | 0.769 |
| walker |  | 3256 | 119 | go package + imports in truststore_java.go |  |  | 0.769 |
| ns | 3277 |  | 431 | Run() dispatch — load + warn loop | 3.6 | 2.1 | 0.713 |
| walker |  | 3608 | 352 | go decl at main.go:49 |  |  | 0.795 |
| ns | 3681 |  | 404 | Run() arg validation — hostname / IP / email / URI / punycode | 3.7 | 2.1 | 0.745 |
| walker |  | 3688 | 80 | go decl body at cert.go:166 |  |  | 0.746 |
| walker |  | 3741 | 53 | README.md section #4 |  |  | 0.746 |
| walker |  | 3781 | 40 | README.md section #22 |  |  | 0.746 |
| walker |  | 3824 | 43 | README.md section #17 |  |  | 0.746 |
| walker |  | 3869 | 45 | README.md section #24 |  |  | 0.746 |
| walker |  | 3983 | 114 | go decl body at truststore_nss.go:120 |  |  | 0.746 |
| walker |  | 4029 | 46 | README.md section #15 |  |  | 0.749 |
| walker |  | 4144 | 115 | go decl body at cert.go:37 |  |  | 0.750 |
| ns | 4254 |  | 573 | main() dispatch — flag combinations + Run call | 3.8 | 2.1 | 0.695 |
| ns | 4322 |  | 68 | truststore_darwin.go package vars | 3.9 |  | 0.689 |
| walker |  | 4354 | 210 | go decl at truststore_nss.go:17 |  |  | 0.692 |
| walker |  | 4401 | 47 | README.md section #19 |  |  | 0.692 |
| ns | 4433 |  | 111 | truststore_linux.go package vars | 3.10 |  | 0.682 |
| walker |  | 4469 | 68 | README.md section #7 |  |  | 0.682 |
| ns | 4507 |  | 74 | truststore_windows.go FirefoxProfiles + cert-help vars | 3.11 |  | 0.677 |
| ns | 4681 |  | 174 | truststore_windows.go DLL var block | 3.12 |  | 0.668 |
| walker |  | 4723 | 254 | go decl body at main.go:240 |  |  | 0.717 |
| walker |  | 4772 | 49 | README.md section #21 |  |  | 0.717 |
| walker |  | 4848 | 76 | README.md section #5 |  |  | 0.717 |
| walker |  | 4980 | 132 | go decl body at truststore_java.go:81 |  |  | 0.717 |
| ns | 5049 |  | 368 | truststore_linux.go init() — distro autodetect | 3.13 | 2.5 | 0.694 |
| walker |  | 5187 | 207 | README.md section #12 |  |  | 0.694 |
| ns | 5430 |  | 381 | truststore_java.go package vars + init | 3.14 | 2.7 | 0.664 |
| walker |  | 5440 | 253 | go package + imports in cert.go |  |  | 0.668 |
| walker |  | 5588 | 148 | go decl body at truststore_java.go:110 |  |  | 0.668 |
| ns | 5698 |  | 268 | truststore_nss.go package vars | 3.15 |  | 0.681 |
| walker |  | 5724 | 136 | go decl body at truststore_java.go:94 |  |  | 0.681 |
| walker |  | 5862 | 138 | go decl names surface in truststore_darwin.go |  |  | 0.691 |
| walker |  | 5862 | 0 | go decl at truststore_darwin.go:25 |  |  | 0.691 |
| walker |  | 5862 | 0 | go decl at truststore_darwin.go:52 |  |  | 0.691 |
| walker |  | 5862 | 0 | go decl at truststore_darwin.go:105 |  |  | 0.691 |
| walker |  | 5871 | 9 | go decl at truststore_darwin.go:18 |  |  | 0.693 |
| walker |  | 5895 | 24 | go decl doc at truststore_darwin.go:25 |  |  | 0.693 |
| walker |  | 6036 | 141 | go decl body at truststore_nss.go:73 |  |  | 0.693 |
| ns | 6085 |  | 387 | truststore_nss.go init() — certutil discovery | 3.16 | 2.8 | 0.666 |
| walker |  | 6183 | 147 | go decl names surface in truststore_linux.go |  |  | 0.681 |
| walker |  | 6183 | 0 | go decl at truststore_linux.go:27 |  |  | 0.681 |
| walker |  | 6183 | 0 | go decl at truststore_linux.go:51 |  |  | 0.681 |
| walker |  | 6183 | 0 | go decl at truststore_linux.go:55 |  |  | 0.681 |
| walker |  | 6183 | 0 | go decl at truststore_linux.go:77 |  |  | 0.681 |
| walker |  | 6212 | 29 | go decl at truststore_linux.go:17 |  |  | 0.687 |
| walker |  | 6235 | 23 | go decl body at truststore_linux.go:51 |  |  | 0.687 |
| walker |  | 6382 | 147 | go decl body at truststore_nss.go:106 |  |  | 0.687 |
| walker |  | 6437 | 55 | README.md section #16 |  |  | 0.687 |
| ns | 6468 |  | 383 | cert.go imports + userAndHostname init() | 4.1 | 2.2 | 0.705 |
| walker |  | 6542 | 105 | README.md section #3 |  |  | 0.705 |
| walker |  | 6651 | 109 | README.md section #6 |  |  | 0.705 |
| ns | 6698 |  | 230 | makeCert template literal — validity + Subject + KeyUsage | 4.2 | 2.2 | 0.693 |
| walker |  | 7039 | 388 | go decl body at main.go:307 |  |  | 0.695 |
| ns | 7045 |  | 347 | makeCert classification + ExtKeyUsage loop | 4.3 | 2.2 | 0.680 |
| walker |  | 7046 | 7 | listing of '.github' |  |  | 0.680 |
| walker |  | 7054 | 8 | listing of '.github/workflows' |  |  | 0.680 |
| walker |  | 7146 | 92 | README.md section #18 |  |  | 0.680 |
| ns | 7326 |  | 281 | newCA template literal — root CA defaults | 4.4 | 2.2 | 0.666 |
| walker |  | 7361 | 215 | go decl body at cert.go:148 |  |  | 0.666 |
| walker |  | 7576 | 215 | go decl body at truststore_nss.go:131 |  |  | 0.666 |
| ns | 7594 |  | 268 | fileNames body — output path derivation | 4.5 | 2.2 | 0.650 |
| walker |  | 7675 | 99 | README.md section #23 |  |  | 0.650 |
| walker |  | 7750 | 75 | go package + imports in truststore_linux.go |  |  | 0.650 |
| ns | 7990 |  | 396 | loadCA body — CA load + keyless mode | 4.6 | 2.2 | 0.635 |
| walker |  | 8004 | 254 | go decl body at truststore_nss.go:89 |  |  | 0.635 |
| walker |  | 8138 | 134 | README.md section #2 |  |  | 0.635 |
| ns | 8174 |  | 184 | generateKey + randomSerialNumber + caUniqueName | 4.7 | 2.2 | 0.640 |
| walker |  | 8399 | 261 | go decl body at cert.go:176 |  |  | 0.666 |
| ns | 8554 |  | 380 | makeCertFromCSR template + CSR-omits defaults | 4.8 | 2.2 | 0.652 |
| walker |  | 8960 | 561 | go decl body at main.go:267 |  |  | 0.655 |
| ns | 9122 |  | 568 | install() body — orchestrate all backends | 5.1 | 2.1 | 0.670 |
| walker |  | 9254 | 294 | go decl body at truststore_java.go:31 |  |  | 0.701 |
| walker |  | 9353 | 99 | README.md section #20 |  |  | 0.701 |
| walker |  | 9441 | 88 | go package + imports in truststore_darwin.go |  |  | 0.701 |
| ns | 9517 |  | 395 | uninstall() body | 5.2 | 2.1 | 0.708 |
| walker |  | 9809 | 368 | go decl names surface in truststore_windows.go |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:35 |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:54 |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:71 |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:83 |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:91 |  |  | 0.729 |
| walker |  | 9809 | 0 | go decl at truststore_windows.go:107 |  |  | 0.729 |
| walker |  | 9818 | 9 | go decl at truststore_windows.go:19 |  |  | 0.730 |
| walker |  | 9827 | 9 | go decl at truststore_windows.go:25 |  |  | 0.731 |
| walker |  | 9890 | 63 | go decl body at truststore_windows.go:83 |  |  | 0.731 |
| ns | 9926 |  | 409 | checkPlatform body + helpers (pathExists/binaryExists/commandWithSudo) | 5.3 | 2.1 | 0.734 |
| walker |  | 9990 | 100 | go package + imports in truststore_windows.go |  |  | 0.734 |
