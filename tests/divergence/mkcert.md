Score(3000)=0.839 I=0.907 C=0.775 ns_rows≤3K=19/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.582/0.640/0.773/0.839/0.804/0.702/0.646

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 38 |  | 38 | README title and one-sentence definition of mkcert | 1.1 |  | 0.000 |
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 88 | 38 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 88 |  | 50 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 117 | 29 | GoMod::Identity { file: go.mod } |  |  | 1.000 |
| walker |  | 140 | 23 | Code::CodeKey { rung: ModuleDoc, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| walker |  | 147 | 7 | Fs::DirListing { dir: .github } |  |  | 0.958 |
| ns | 147 |  | 59 | Package doc line, module path and Go version | 1.3 |  | 0.958 |
| walker |  | 155 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.960 |
| walker |  | 164 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.964 |
| walker |  | 249 | 85 | GoMod::File { file: go.mod } |  |  | 0.741 |
| ns | 249 |  | 102 | main.go declaration roster, part A: usage strings, Version, main, the CAROOT filename consts, the mkcert type, Run | 1.4 |  | 0.741 |
| walker |  | 384 | 135 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.609 |
| ns | 384 |  | 135 | main.go declaration roster, part B: every remaining top-level function | 1.5 |  | 0.609 |
| ns | 551 |  | 167 | cert.go declaration roster: every top-level symbol | 1.6 |  | 0.533 |
| walker |  | 606 | 222 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.533 |
| ns | 702 |  | 151 | Per-platform truststore contract, part 1: truststore_darwin.go and truststore_linux.go declaration rosters | 1.7 |  | 0.470 |
| walker |  | 757 | 151 | Code::CodeKey { rung: Names, file: truststore_java.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 766 | 9 | Code::CodeKey { rung: Decl, file: truststore_java.go, decl: 1, sub: 0, line: 21 } |  |  | 0.473 |
| ns | 824 |  | 122 | truststore_windows.go declaration roster, including the windowsRootStore type and crypt32 proc vars | 1.8 | 1.7 | 0.432 |
| ns | 1007 |  | 183 | Cross-platform truststore rosters: truststore_nss.go and truststore_java.go | 1.9 |  | 0.415 |
| walker |  | 1061 | 295 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 1146 |  | 139 | Every README heading below the title, H2 through H4 | 1.10 |  | 0.630 |
| ns | 1170 |  | 24 | Complete .github tree listing | 1.11 |  | 0.644 |
| walker |  | 1235 | 174 | Code::CodeKey { rung: Decl, file: main.go, decl: 7, sub: 0, line: 155 } |  |  | 0.658 |
| ns | 1407 |  | 237 | The complete flag set declared in main() | 2.1 | 1.4 | 0.615 |
| walker |  | 1433 | 198 | Code::CodeKey { rung: Decl, file: main.go, decl: 1, sub: 0, line: 30 } |  |  | 0.632 |
| ns | 1605 |  | 198 | shortUsage: the default help text with its four worked examples | 2.2 | 1.4 | 0.665 |
| ns | 1763 |  | 158 | The two environment variables: $CAROOT and $TRUST_STORES | 2.3 |  | 0.632 |
| walker |  | 1787 | 354 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 49 } |  |  | 0.703 |
| ns | 1954 |  | 191 | advancedUsage: per-flag descriptions for the output-path, -client, -ecdsa, -pkcs12 and -csr options | 2.4 | 1.4 | 0.722 |
| walker |  | 1962 | 175 | Code::CodeKey { rung: Names, file: truststore_nss.go, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| ns | 2128 |  | 174 | Fields of the mkcert struct | 2.5 | 1.4 | 0.774 |
| walker |  | 2172 | 210 | Code::CodeKey { rung: Decl, file: truststore_nss.go, decl: 1, sub: 0, line: 17 } |  |  | 0.786 |
| walker |  | 2321 | 149 | Code::CodeKey { rung: Names, file: truststore_linux.go, decl: 0, sub: 0, line: 0 } |  |  | 0.795 |
| ns | 2335 |  | 207 | README: the supported root stores list and the TRUST_STORES subset note | 2.6 |  | 0.762 |
| walker |  | 2350 | 29 | Code::CodeKey { rung: Decl, file: truststore_linux.go, decl: 1, sub: 0, line: 17 } |  |  | 0.765 |
| walker |  | 2490 | 140 | Code::CodeKey { rung: Names, file: truststore_darwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.804 |
| walker |  | 2501 | 11 | Code::CodeKey { rung: Decl, file: truststore_darwin.go, decl: 1, sub: 0, line: 18 } |  |  | 0.809 |
| ns | 2566 |  | 231 | The per-platform constant blocks of all three GOOS truststore files | 2.7 | 1.8 | 0.795 |
| walker |  | 2769 | 268 | Code::CodeKey { rung: Decl, file: truststore_darwin.go, decl: 4, sub: 0, line: 27 } |  |  | 0.795 |
| walker |  | 2791 | 22 | Code::CodeKey { rung: Doc, file: truststore_darwin.go, decl: 2, sub: 0, line: 25 } |  |  | 0.795 |
| ns | 2893 |  | 327 | NSS and Java constant blocks: the nssDBs / firefoxPaths search lists and the keytool state vars | 2.8 | 1.9 | 0.806 |
| walker |  | 3007 | 216 | Code::CodeKey { rung: Names, file: cert.go, decl: 0, sub: 0, line: 0 } |  |  | 0.846 |
| walker |  | 3024 | 17 | Code::CodeKey { rung: Doc, file: cert.go, decl: 9, sub: 0, line: 282 } |  |  | 0.846 |
| walker |  | 3043 | 19 | Code::CodeKey { rung: Body, file: cert.go, decl: 11, sub: 0, line: 366 } |  |  | 0.846 |
| ns | 3259 |  | 366 | Linux platform detection: init() choosing the distro trust anchor directory and certutil install hint | 2.9 | 1.7 | 0.805 |
| walker |  | 3413 | 370 | Code::CodeKey { rung: Names, file: truststore_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.847 |
| walker |  | 3422 | 9 | Code::CodeKey { rung: Decl, file: truststore_windows.go, decl: 2, sub: 0, line: 25 } |  |  | 0.851 |
| walker |  | 3431 | 9 | Code::CodeKey { rung: Decl, file: truststore_windows.go, decl: 1, sub: 0, line: 19 } |  |  | 0.857 |
| ns | 3644 |  | 385 | NSS detection: init() setting hasNSS, hasCertutil and certutilPath | 2.10 | 1.9 | 0.803 |
| walker |  | 3717 | 286 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 0, line: 87 } |  |  | 0.837 |
| walker |  | 3912 | 195 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.837 |
| walker |  | 3937 | 25 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 3, sub: 0, line: 51 } |  |  | 0.837 |
| ns | 3943 |  | 299 | Java detection: init() resolving JAVA_HOME, keytool and the cacerts keystore | 2.11 | 1.9 | 0.801 |
| walker |  | 3978 | 41 | Code::CodeKey { rung: Doc, file: truststore_java.go, decl: 6, sub: 0, line: 110 } |  |  | 0.801 |
| walker |  | 4019 | 41 | Code::CodeKey { rung: Doc, file: truststore_nss.go, decl: 6, sub: 0, line: 120 } |  |  | 0.801 |
| walker |  | 4238 | 219 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 1, line: 87 } |  |  | 0.803 |
| ns | 4374 |  | 431 | Run(): CAROOT setup, loadCA, and the install/uninstall/warn dispatch | 3.1 | 1.4 | 0.760 |
| walker |  | 4430 | 192 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 2, line: 87 } |  |  | 0.761 |
| walker |  | 4585 | 155 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 3, line: 87 } |  |  | 0.763 |
| ns | 4776 |  | 402 | Run(): the CSR branch and the hostname/IP/email/URI argument validation | 3.2 | 3.1 | 0.726 |
| walker |  | 4792 | 207 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.752 |
| walker |  | 4860 | 68 | Code::CodeKey { rung: Body, file: cert.go, decl: 7, sub: 0, line: 202 } |  |  | 0.752 |
| ns | 5035 |  | 259 | getCAROOT(): where the local CA is stored on each OS | 3.3 | 1.5 | 0.726 |
| walker |  | 5155 | 295 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.726 |
| ns | 5189 |  | 154 | storeEnabled() and checkPlatform(): the TRUST_STORES gate and the system-store check | 3.4 | 1.5 | 0.708 |
| walker |  | 5403 | 248 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.709 |
| ns | 5543 |  | 354 | main(): flag conflict validation and construction of the mkcert value | 3.5 | 2.1 | 0.720 |
| walker |  | 5642 | 239 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.720 |
| ns | 5695 |  | 152 | main(): the flag.Usage override, -help and -version handling | 3.6 | 2.1 | 0.727 |
| walker |  | 5703 | 61 | Code::CodeKey { rung: Doc, file: main.go, decl: 3, sub: 0, line: 85 } |  |  | 0.727 |
| ns | 5876 |  | 181 | commandWithSudo(): how every privileged trust-store command is wrapped | 3.7 | 1.5 | 0.716 |
| walker |  | 5934 | 231 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.716 |
| ns | 5961 |  | 85 | generateKey(): the key algorithm and sizes | 4.1 | 1.6 | 0.709 |
| walker |  | 6016 | 82 | Code::CodeKey { rung: Body, file: cert.go, decl: 5, sub: 0, line: 166 } |  |  | 0.719 |
| ns | 6227 |  | 266 | fileNames(): the output filename convention | 4.2 | 1.6 | 0.699 |
| walker |  | 6270 | 254 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.702 |
| walker |  | 6387 | 117 | Code::CodeKey { rung: Body, file: cert.go, decl: 2, sub: 0, line: 37 } |  |  | 0.703 |
| ns | 6558 |  | 331 | makeCert(): the leaf certificate template and its 2-year-3-month validity | 4.3 | 1.6 | 0.685 |
| walker |  | 6795 | 408 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.685 |
| walker |  | 6860 | 65 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 7, sub: 0, line: 83 } |  |  | 0.685 |
| walker |  | 6937 | 77 | Code::CodeKey { rung: Body, file: truststore_darwin.go, decl: 6, sub: 0, line: 105 } |  |  | 0.685 |
| ns | 6981 |  | 423 | makeCert(): SAN classification and extended key usages | 4.4 | 4.3 | 0.667 |
| walker |  | 7071 | 134 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 4, sub: 0, line: 81 } |  |  | 0.667 |
| ns | 7232 |  | 251 | makeCert(): signing, and the two write paths (PEM pair vs PKCS#12) with their file modes | 4.5 | 4.4 | 0.657 |
| walker |  | 7288 | 217 | Code::CodeKey { rung: Body, file: cert.go, decl: 4, sub: 0, line: 148 } |  |  | 0.657 |
| walker |  | 7426 | 138 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 5, sub: 0, line: 94 } |  |  | 0.659 |
| ns | 7515 |  | 283 | newCA(): the root certificate template | 4.6 | 1.6 | 0.645 |
| walker |  | 7542 | 116 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 6, sub: 0, line: 120 } |  |  | 0.645 |
| walker |  | 7664 | 122 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 6, sub: 0, line: 71 } |  |  | 0.645 |
| ns | 7759 |  | 244 | newCA(): writing rootCA-key.pem at 0400 and rootCA.pem at 0644 | 4.7 | 4.6 | 0.636 |
| walker |  | 7922 | 258 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 5, sub: 0, line: 77 } |  |  | 0.637 |
| ns | 7999 |  | 240 | loadCA(): create-on-demand, PEM parsing, and keyless mode | 4.8 | 1.6 | 0.626 |
| walker |  | 8072 | 150 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 6, sub: 0, line: 110 } |  |  | 0.626 |
| walker |  | 8093 | 21 | Code::CodeKey { rung: Body, file: main.go, decl: 16, sub: 0, line: 370 } |  |  | 0.626 |
| ns | 8212 |  | 213 | The identity strings: userAndHostname init, randomSerialNumber, caUniqueName | 4.9 | 1.6 | 0.632 |
| walker |  | 8236 | 143 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 3, sub: 0, line: 73 } |  |  | 0.632 |
| walker |  | 8258 | 22 | Code::CodeKey { rung: Body, file: main.go, decl: 17, sub: 0, line: 375 } |  |  | 0.632 |
| ns | 8476 |  | 264 | makeCertFromCSR(): the template fix-ups applied to a supplied CSR | 4.10 | 1.6 | 0.622 |
| walker |  | 8521 | 263 | Code::CodeKey { rung: Body, file: cert.go, decl: 6, sub: 0, line: 176 } |  |  | 0.648 |
| ns | 8605 |  | 129 | printHosts(): the certificate name list and the second-level wildcard warning | 4.11 | 1.6 | 0.652 |
| walker |  | 8672 | 151 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 4, sub: 0, line: 54 } |  |  | 0.652 |
| walker |  | 8821 | 149 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 5, sub: 0, line: 106 } |  |  | 0.654 |
| ns | 8856 |  | 251 | The macOS and Linux system-store install commands | 5.1 | 1.7 | 0.645 |
| walker |  | 9083 | 262 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 4, sub: 0, line: 55 } |  |  | 0.649 |
| ns | 9088 |  | 232 | The Windows root-store install and uninstall, via crypt32 | 5.2 | 1.8 | 0.646 |
| walker |  | 9118 | 35 | Code::CodeKey { rung: Body, file: main.go, decl: 14, sub: 0, line: 358 } |  |  | 0.646 |
| walker |  | 9328 | 210 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 3, sub: 0, line: 35 } |  |  | 0.658 |
| ns | 9331 |  | 243 | installNSS(): the certutil invocation and its failure advice | 5.3 | 1.9 | 0.652 |
| walker |  | 9372 | 44 | Code::CodeKey { rung: Body, file: main.go, decl: 15, sub: 0, line: 364 } |  |  | 0.652 |
| ns | 9468 |  | 137 | installJava(): the keytool -importcert argument list | 5.4 | 1.9 | 0.655 |
| ns | 9610 |  | 142 | forEachNSSProfile(): how Firefox/Chromium profiles are discovered and which DB format is used | 5.5 | 1.9 | 0.649 |
| walker |  | 9668 | 296 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 2, sub: 0, line: 31 } |  |  | 0.673 |
| ns | 9785 |  | 175 | The macOS and Linux uninstall commands, including the legacy filename cleanup | 5.6 | 5.1 | 0.676 |
| ns | 9930 |  | 145 | go.mod: the direct and indirect dependency set | 6.1 | 1.3 | 0.674 |
| ns | 9974 |  | 44 | README: building from source with the version stamp | 6.2 |  | 0.674 |
| walker |  | 9989 | 321 | Code::CodeKey { rung: Body, file: cert.go, decl: 9, sub: 0, line: 282 } |  |  | 0.691 |
