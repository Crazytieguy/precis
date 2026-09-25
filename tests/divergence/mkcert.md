Score(3000)=0.837 I=0.910 C=0.770 ns_rows≤3K=19/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.630/0.645/0.837/0.767/0.674/0.628

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 38 |  | 38 | README title and one-sentence definition of mkcert | 1.1 |  | 0.000 |
| walker |  | 50 | 50 | listing of '.' |  |  | 0.000 |
| walker |  | 73 | 23 | go module doc main.go |  |  | 0.000 |
| ns | 88 |  | 50 | Complete root directory listing | 1.2 |  | 0.616 |
| walker |  | 102 | 29 | go module identity in go.mod |  |  | 0.647 |
| walker |  | 140 | 38 | README headline in README.md |  |  | 1.000 |
| walker |  | 147 | 7 | listing of '.github' |  |  | 0.958 |
| ns | 147 |  | 59 | Package doc line, module path and Go version | 1.3 |  | 0.958 |
| walker |  | 155 | 8 | listing of '.github/workflows' |  |  | 0.960 |
| ns | 249 |  | 102 | main.go declaration roster, part A: usage strings, Version, main, the CAROOT filename consts, the mkcert type, Run | 1.4 |  | 0.737 |
| ns | 384 |  | 135 | main.go declaration roster, part B: every remaining top-level function | 1.5 |  | 0.598 |
| walker |  | 450 | 295 | go names main.go |  |  | 0.900 |
| walker |  | 471 | 21 | go body main.go:370 |  |  | 0.900 |
| walker |  | 493 | 22 | go body main.go:375 |  |  | 0.900 |
| ns | 551 |  | 167 | cert.go declaration roster: every top-level symbol | 1.6 |  | 0.788 |
| walker |  | 667 | 174 | go decl main.go:155 |  |  | 0.809 |
| ns | 702 |  | 151 | Per-platform truststore contract, part 1: truststore_darwin.go and truststore_linux.go declaration rosters | 1.7 |  | 0.713 |
| ns | 824 |  | 122 | truststore_windows.go declaration roster, including the windowsRootStore type and crypt32 proc vars | 1.8 | 1.7 | 0.652 |
| walker |  | 865 | 198 | go decl main.go:30 |  |  | 0.673 |
| walker |  | 1005 | 140 | go names truststore_darwin.go |  |  | 0.707 |
| ns | 1007 |  | 183 | Cross-platform truststore rosters: truststore_nss.go and truststore_java.go | 1.9 |  | 0.630 |
| walker |  | 1016 | 11 | go decl truststore_darwin.go:18 |  |  | 0.636 |
| walker |  | 1038 | 22 | go doc truststore_darwin.go:25 |  |  | 0.636 |
| ns | 1146 |  | 139 | Every README heading below the title, H2 through H4 | 1.10 |  | 0.590 |
| ns | 1170 |  | 24 | Complete .github tree listing | 1.11 |  | 0.589 |
| walker |  | 1187 | 149 | go names truststore_linux.go |  |  | 0.644 |
| walker |  | 1216 | 29 | go decl truststore_linux.go:17 |  |  | 0.653 |
| walker |  | 1367 | 151 | go names truststore_java.go |  |  | 0.670 |
| walker |  | 1376 | 9 | go decl truststore_java.go:21 |  |  | 0.674 |
| walker |  | 1401 | 25 | go body truststore_linux.go:51 |  |  | 0.674 |
| ns | 1407 |  | 237 | The complete flag set declared in main() | 2.1 | 1.4 | 0.630 |
| walker |  | 1551 | 150 | go module file go.mod |  |  | 0.631 |
| ns | 1605 |  | 198 | shortUsage: the default help text with its four worked examples | 2.2 | 1.4 | 0.665 |
| walker |  | 1726 | 175 | go names truststore_nss.go |  |  | 0.719 |
| walker |  | 1761 | 35 | go body main.go:358 |  |  | 0.719 |
| ns | 1763 |  | 158 | The two environment variables: $CAROOT and $TRUST_STORES | 2.3 |  | 0.684 |
| ns | 1954 |  | 191 | advancedUsage: per-flag descriptions for the output-path, -client, -ecdsa, -pkcs12 and -csr options | 2.4 | 1.4 | 0.645 |
| walker |  | 2115 | 354 | go decl main.go:49 |  |  | 0.769 |
| ns | 2128 |  | 174 | Fields of the mkcert struct | 2.5 | 1.4 | 0.778 |
| walker |  | 2331 | 216 | go names cert.go |  |  | 0.826 |
| ns | 2335 |  | 207 | README: the supported root stores list and the TRUST_STORES subset note | 2.6 |  | 0.793 |
| walker |  | 2348 | 17 | go doc cert.go:282 |  |  | 0.793 |
| walker |  | 2367 | 19 | go body cert.go:366 |  |  | 0.793 |
| walker |  | 2502 | 135 | headings outline in README.md |  |  | 0.832 |
| walker |  | 2543 | 41 | README.md section #1 |  |  | 0.832 |
| ns | 2566 |  | 231 | The per-platform constant blocks of all three GOOS truststore files | 2.7 | 1.8 | 0.818 |
| walker |  | 2753 | 210 | go decl truststore_nss.go:17 |  |  | 0.829 |
| walker |  | 2794 | 41 | go doc truststore_java.go:110 |  |  | 0.829 |
| walker |  | 2835 | 41 | go doc truststore_nss.go:120 |  |  | 0.829 |
| ns | 2893 |  | 327 | NSS and Java constant blocks: the nssDBs / firefoxPaths search lists and the keytool state vars | 2.8 | 1.9 | 0.837 |
| walker |  | 3103 | 268 | go decl truststore_darwin.go:27 |  |  | 0.837 |
| walker |  | 3123 | 20 | README.md section #11 |  |  | 0.837 |
| ns | 3259 |  | 366 | Linux platform detection: init() choosing the distro trust anchor directory and certutil install hint | 2.9 | 1.7 | 0.797 |
| walker |  | 3493 | 370 | go names truststore_windows.go |  |  | 0.839 |
| walker |  | 3502 | 9 | go decl truststore_windows.go:25 |  |  | 0.843 |
| walker |  | 3511 | 9 | go decl truststore_windows.go:19 |  |  | 0.849 |
| walker |  | 3555 | 44 | go body main.go:364 |  |  | 0.849 |
| walker |  | 3610 | 55 | go body main.go:336 |  |  | 0.849 |
| ns | 3644 |  | 385 | NSS detection: init() setting hasNSS, hasCertutil and certutilPath | 2.10 | 1.9 | 0.796 |
| walker |  | 3671 | 61 | go doc main.go:85 |  |  | 0.796 |
| walker |  | 3764 | 93 | go body main.go:345 |  |  | 0.798 |
| walker |  | 3829 | 65 | go body truststore_windows.go:83 |  |  | 0.798 |
| walker |  | 3897 | 68 | go body cert.go:202 |  |  | 0.798 |
| ns | 3943 |  | 299 | Java detection: init() resolving JAVA_HOME, keytool and the cacerts keystore | 2.11 | 1.9 | 0.764 |
| walker |  | 3974 | 77 | go body truststore_darwin.go:105 |  |  | 0.764 |
| walker |  | 4056 | 82 | go body cert.go:166 |  |  | 0.765 |
| walker |  | 4222 | 166 | go body main.go:382 |  |  | 0.767 |
| walker |  | 4245 | 23 | README.md section #14 |  |  | 0.767 |
| ns | 4374 |  | 431 | Run(): CAROOT setup, loadCA, and the install/uninstall/warn dispatch | 3.1 | 1.4 | 0.725 |
| walker |  | 4667 | 422 | README.md section #0 |  |  | 0.725 |
| ns | 4776 |  | 402 | Run(): the CSR branch and the hostname/IP/email/URI argument validation | 3.2 | 3.1 | 0.689 |
| walker |  | 4783 | 116 | go body truststore_nss.go:120 |  |  | 0.689 |
| walker |  | 4822 | 39 | README.md section #10 |  |  | 0.689 |
| walker |  | 4939 | 117 | go body cert.go:37 |  |  | 0.691 |
| ns | 5035 |  | 259 | getCAROOT(): where the local CA is stored on each OS | 3.3 | 1.5 | 0.666 |
| walker |  | 5061 | 122 | go body truststore_windows.go:71 |  |  | 0.666 |
| ns | 5189 |  | 154 | storeEnabled() and checkPlatform(): the TRUST_STORES gate and the system-store check | 3.4 | 1.5 | 0.675 |
| walker |  | 5195 | 134 | go body truststore_java.go:81 |  |  | 0.675 |
| walker |  | 5451 | 256 | go body main.go:240 |  |  | 0.714 |
| ns | 5543 |  | 354 | main(): flag conflict validation and construction of the mkcert value | 3.5 | 2.1 | 0.694 |
| walker |  | 5589 | 138 | go body truststore_java.go:94 |  |  | 0.696 |
| walker |  | 5634 | 45 | README.md section #9 |  |  | 0.696 |
| walker |  | 5678 | 44 | README.md section #8 |  |  | 0.696 |
| ns | 5695 |  | 152 | main(): the flag.Usage override, -help and -version handling | 3.6 | 2.1 | 0.682 |
| walker |  | 5821 | 143 | go body truststore_nss.go:73 |  |  | 0.682 |
| ns | 5876 |  | 181 | commandWithSudo(): how every privileged trust-store command is wrapped | 3.7 | 1.5 | 0.688 |
| ns | 5961 |  | 85 | generateKey(): the key algorithm and sizes | 4.1 | 1.6 | 0.690 |
| walker |  | 5970 | 149 | go body truststore_nss.go:106 |  |  | 0.692 |
| walker |  | 6120 | 150 | go body truststore_java.go:110 |  |  | 0.692 |
| ns | 6227 |  | 266 | fileNames(): the output filename convention | 4.2 | 1.6 | 0.674 |
| walker |  | 6271 | 151 | go body truststore_windows.go:54 |  |  | 0.674 |
| walker |  | 6326 | 55 | README.md section #4 |  |  | 0.674 |
| walker |  | 6366 | 40 | README.md section #22 |  |  | 0.674 |
| ns | 6558 |  | 331 | makeCert(): the leaf certificate template and its 2-year-3-month validity | 4.3 | 1.6 | 0.657 |
| walker |  | 6756 | 390 | go body main.go:307 |  |  | 0.660 |
| walker |  | 6966 | 210 | go body truststore_windows.go:35 |  |  | 0.661 |
| ns | 6981 |  | 423 | makeCert(): SAN classification and extended key usages | 4.4 | 4.3 | 0.643 |
| walker |  | 7009 | 43 | README.md section #17 |  |  | 0.643 |
| walker |  | 7222 | 213 | go body truststore_windows.go:91 |  |  | 0.643 |
| ns | 7232 |  | 251 | makeCert(): signing, and the two write paths (PEM pair vs PKCS#12) with their file modes | 4.5 | 4.4 | 0.633 |
| walker |  | 7439 | 217 | go body cert.go:148 |  |  | 0.633 |
| ns | 7515 |  | 283 | newCA(): the root certificate template | 4.6 | 1.6 | 0.620 |
| walker |  | 7656 | 217 | go body truststore_nss.go:131 |  |  | 0.620 |
| walker |  | 7700 | 44 | README.md section #15 |  |  | 0.623 |
| walker |  | 7745 | 45 | README.md section #24 |  |  | 0.623 |
| ns | 7759 |  | 244 | newCA(): writing rootCA-key.pem at 0400 and rootCA.pem at 0644 | 4.7 | 4.6 | 0.614 |
| walker |  | 7792 | 47 | README.md section #19 |  |  | 0.614 |
| walker |  | 7839 | 47 | README.md section #21 |  |  | 0.614 |
| walker |  | 7907 | 68 | README.md section #7 |  |  | 0.614 |
| ns | 7999 |  | 240 | loadCA(): create-on-demand, PEM parsing, and keyless mode | 4.8 | 1.6 | 0.604 |
| walker |  | 8163 | 256 | go body truststore_nss.go:89 |  |  | 0.605 |
| ns | 8212 |  | 213 | The identity strings: userAndHostname init, randomSerialNumber, caUniqueName | 4.9 | 1.6 | 0.612 |
| walker |  | 8421 | 258 | go body truststore_linux.go:77 |  |  | 0.612 |
| ns | 8476 |  | 264 | makeCertFromCSR(): the template fix-ups applied to a supplied CSR | 4.10 | 1.6 | 0.603 |
| ns | 8605 |  | 129 | printHosts(): the certificate name list and the second-level wildcard warning | 4.11 | 1.6 | 0.607 |
| walker |  | 8683 | 262 | go body truststore_linux.go:55 |  |  | 0.608 |
| ns | 8856 |  | 251 | The macOS and Linux system-store install commands | 5.1 | 1.7 | 0.604 |
| walker |  | 8946 | 263 | go body cert.go:176 |  |  | 0.628 |
| walker |  | 9022 | 76 | README.md section #5 |  |  | 0.629 |
| walker |  | 9077 | 55 | README.md section #16 |  |  | 0.629 |
| ns | 9088 |  | 232 | The Windows root-store install and uninstall, via crypt32 | 5.2 | 1.8 | 0.637 |
| ns | 9331 |  | 243 | installNSS(): the certutil invocation and its failure advice | 5.3 | 1.9 | 0.642 |
| ns | 9468 |  | 137 | installJava(): the keytool -importcert argument list | 5.4 | 1.9 | 0.646 |
| walker |  | 9470 | 393 | go body main.go:267 |  |  | 0.648 |
| ns | 9610 |  | 142 | forEachNSSProfile(): how Firefox/Chromium profiles are discovered and which DB format is used | 5.5 | 1.9 | 0.653 |
| walker |  | 9766 | 296 | go body truststore_java.go:31 |  |  | 0.677 |
| ns | 9785 |  | 175 | The macOS and Linux uninstall commands, including the legacy filename cleanup | 5.6 | 5.1 | 0.680 |
| ns | 9930 |  | 145 | go.mod: the direct and indirect dependency set | 6.1 | 1.3 | 0.683 |
| walker |  | 9973 | 207 | README.md section #12 |  |  | 0.698 |
| ns | 9974 |  | 44 | README: building from source with the version stamp | 6.2 |  | 0.699 |
