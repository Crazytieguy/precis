Score(3000)=0.796 I=0.873 C=0.726 ns_rows≤3K=19/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.558/0.607/0.727/0.796/0.763/0.679/0.651

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 38 |  | 38 | README title and one-sentence definition of mkcert | 1.1 |  | 0.000 |
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 79 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.000 |
| ns | 88 |  | 50 | Complete root directory listing | 1.2 |  | 0.618 |
| walker |  | 102 | 23 | Code::CodeKey { rung: ModuleDoc, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 140 | 38 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 147 | 7 | Fs::DirListing { dir: .github } |  |  | 0.958 |
| ns | 147 |  | 59 | Package doc line, module path and Go version | 1.3 |  | 0.958 |
| walker |  | 155 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.960 |
| walker |  | 240 | 85 | GoMod::File { file: go.mod } |  |  | 0.961 |
| ns | 249 |  | 102 | main.go declaration roster, part A: usage strings, Version, main, the CAROOT filename consts, the mkcert type, Run | 1.4 |  | 0.738 |
| walker |  | 375 | 135 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.749 |
| ns | 384 |  | 135 | main.go declaration roster, part B: every remaining top-level function | 1.5 |  | 0.607 |
| ns | 551 |  | 167 | cert.go declaration roster: every top-level symbol | 1.6 |  | 0.531 |
| walker |  | 597 | 222 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.531 |
| ns | 702 |  | 151 | Per-platform truststore contract, part 1: truststore_darwin.go and truststore_linux.go declaration rosters | 1.7 |  | 0.468 |
| walker |  | 748 | 151 | Code::CodeKey { rung: Names, file: truststore_java.go, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 757 | 9 | Code::CodeKey { rung: Decl, file: truststore_java.go, decl: 1, sub: 0, line: 21 } |  |  | 0.471 |
| walker |  | 798 | 41 | Code::CodeKey { rung: Doc, file: truststore_java.go, decl: 6, sub: 0, line: 110 } |  |  | 0.471 |
| ns | 824 |  | 122 | truststore_windows.go declaration roster, including the windowsRootStore type and crypt32 proc vars | 1.8 | 1.7 | 0.431 |
| ns | 1007 |  | 183 | Cross-platform truststore rosters: truststore_nss.go and truststore_java.go | 1.9 |  | 0.413 |
| walker |  | 1093 | 295 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 1114 | 21 | Code::CodeKey { rung: Body, file: main.go, decl: 16, sub: 0, line: 370 } |  |  | 0.600 |
| walker |  | 1136 | 22 | Code::CodeKey { rung: Body, file: main.go, decl: 17, sub: 0, line: 375 } |  |  | 0.600 |
| ns | 1146 |  | 139 | Every README heading below the title, H2 through H4 | 1.10 |  | 0.628 |
| ns | 1170 |  | 24 | Complete .github tree listing | 1.11 |  | 0.624 |
| walker |  | 1310 | 174 | Code::CodeKey { rung: Decl, file: main.go, decl: 7, sub: 0, line: 155 } |  |  | 0.638 |
| ns | 1407 |  | 237 | The complete flag set declared in main() | 2.1 | 1.4 | 0.596 |
| walker |  | 1508 | 198 | Code::CodeKey { rung: Decl, file: main.go, decl: 1, sub: 0, line: 30 } |  |  | 0.613 |
| walker |  | 1543 | 35 | Code::CodeKey { rung: Body, file: main.go, decl: 14, sub: 0, line: 358 } |  |  | 0.613 |
| ns | 1605 |  | 198 | shortUsage: the default help text with its four worked examples | 2.2 | 1.4 | 0.648 |
| ns | 1763 |  | 158 | The two environment variables: $CAROOT and $TRUST_STORES | 2.3 |  | 0.617 |
| walker |  | 1897 | 354 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 49 } |  |  | 0.689 |
| ns | 1954 |  | 191 | advancedUsage: per-flag descriptions for the output-path, -client, -ecdsa, -pkcs12 and -csr options | 2.4 | 1.4 | 0.708 |
| walker |  | 1958 | 61 | Code::CodeKey { rung: Doc, file: main.go, decl: 3, sub: 0, line: 85 } |  |  | 0.708 |
| ns | 2128 |  | 174 | Fields of the mkcert struct | 2.5 | 1.4 | 0.721 |
| walker |  | 2133 | 175 | Code::CodeKey { rung: Names, file: truststore_nss.go, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 2335 |  | 207 | README: the supported root stores list and the TRUST_STORES subset note | 2.6 |  | 0.731 |
| walker |  | 2343 | 210 | Code::CodeKey { rung: Decl, file: truststore_nss.go, decl: 1, sub: 0, line: 17 } |  |  | 0.743 |
| walker |  | 2384 | 41 | Code::CodeKey { rung: Doc, file: truststore_nss.go, decl: 6, sub: 0, line: 120 } |  |  | 0.743 |
| walker |  | 2533 | 149 | Code::CodeKey { rung: Names, file: truststore_linux.go, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 2562 | 29 | Code::CodeKey { rung: Decl, file: truststore_linux.go, decl: 1, sub: 0, line: 17 } |  |  | 0.753 |
| ns | 2566 |  | 231 | The per-platform constant blocks of all three GOOS truststore files | 2.7 | 1.8 | 0.730 |
| walker |  | 2587 | 25 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 3, sub: 0, line: 51 } |  |  | 0.730 |
| walker |  | 2727 | 140 | Code::CodeKey { rung: Names, file: truststore_darwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| walker |  | 2738 | 11 | Code::CodeKey { rung: Decl, file: truststore_darwin.go, decl: 1, sub: 0, line: 18 } |  |  | 0.785 |
| walker |  | 2760 | 22 | Code::CodeKey { rung: Doc, file: truststore_darwin.go, decl: 2, sub: 0, line: 25 } |  |  | 0.785 |
| ns | 2893 |  | 327 | NSS and Java constant blocks: the nssDBs / firefoxPaths search lists and the keytool state vars | 2.8 | 1.9 | 0.796 |
| walker |  | 3028 | 268 | Code::CodeKey { rung: Decl, file: truststore_darwin.go, decl: 4, sub: 0, line: 27 } |  |  | 0.796 |
| walker |  | 3105 | 77 | Code::CodeKey { rung: Body, file: truststore_darwin.go, decl: 6, sub: 0, line: 105 } |  |  | 0.796 |
| ns | 3259 |  | 366 | Linux platform detection: init() choosing the distro trust anchor directory and certutil install hint | 2.9 | 1.7 | 0.758 |
| walker |  | 3321 | 216 | Code::CodeKey { rung: Names, file: cert.go, decl: 0, sub: 0, line: 0 } |  |  | 0.796 |
| walker |  | 3338 | 17 | Code::CodeKey { rung: Doc, file: cert.go, decl: 9, sub: 0, line: 282 } |  |  | 0.796 |
| walker |  | 3357 | 19 | Code::CodeKey { rung: Body, file: cert.go, decl: 11, sub: 0, line: 366 } |  |  | 0.796 |
| walker |  | 3425 | 68 | Code::CodeKey { rung: Body, file: cert.go, decl: 7, sub: 0, line: 202 } |  |  | 0.797 |
| walker |  | 3507 | 82 | Code::CodeKey { rung: Body, file: cert.go, decl: 5, sub: 0, line: 166 } |  |  | 0.798 |
| walker |  | 3624 | 117 | Code::CodeKey { rung: Body, file: cert.go, decl: 2, sub: 0, line: 37 } |  |  | 0.799 |
| ns | 3644 |  | 385 | NSS detection: init() setting hasNSS, hasCertutil and certutilPath | 2.10 | 1.9 | 0.749 |
| ns | 3943 |  | 299 | Java detection: init() resolving JAVA_HOME, keytool and the cacerts keystore | 2.11 | 1.9 | 0.717 |
| walker |  | 3994 | 370 | Code::CodeKey { rung: Names, file: truststore_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| walker |  | 4003 | 9 | Code::CodeKey { rung: Decl, file: truststore_windows.go, decl: 2, sub: 0, line: 25 } |  |  | 0.758 |
| walker |  | 4012 | 9 | Code::CodeKey { rung: Decl, file: truststore_windows.go, decl: 1, sub: 0, line: 19 } |  |  | 0.763 |
| walker |  | 4077 | 65 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 7, sub: 0, line: 83 } |  |  | 0.763 |
| walker |  | 4193 | 116 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 6, sub: 0, line: 120 } |  |  | 0.763 |
| walker |  | 4315 | 122 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 6, sub: 0, line: 71 } |  |  | 0.763 |
| ns | 4374 |  | 431 | Run(): CAROOT setup, loadCA, and the install/uninstall/warn dispatch | 3.1 | 1.4 | 0.722 |
| walker |  | 4449 | 134 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 4, sub: 0, line: 81 } |  |  | 0.723 |
| walker |  | 4735 | 286 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 0, line: 87 } |  |  | 0.754 |
| ns | 4776 |  | 402 | Run(): the CSR branch and the hostname/IP/email/URI argument validation | 3.2 | 3.1 | 0.717 |
| walker |  | 4873 | 138 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 5, sub: 0, line: 94 } |  |  | 0.719 |
| walker |  | 5016 | 143 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 3, sub: 0, line: 73 } |  |  | 0.719 |
| ns | 5035 |  | 259 | getCAROOT(): where the local CA is stored on each OS | 3.3 | 1.5 | 0.694 |
| ns | 5189 |  | 154 | storeEnabled() and checkPlatform(): the TRUST_STORES gate and the system-store check | 3.4 | 1.5 | 0.677 |
| walker |  | 5321 | 305 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 1, line: 87 } |  |  | 0.679 |
| ns | 5543 |  | 354 | main(): flag conflict validation and construction of the mkcert value | 3.5 | 2.1 | 0.663 |
| walker |  | 5582 | 261 | Code::CodeKey { rung: Decl, file: main.go, decl: 4, sub: 2, line: 87 } |  |  | 0.695 |
| ns | 5695 |  | 152 | main(): the flag.Usage override, -help and -version handling | 3.6 | 2.1 | 0.703 |
| walker |  | 5799 | 217 | Code::CodeKey { rung: Body, file: cert.go, decl: 4, sub: 0, line: 148 } |  |  | 0.703 |
| ns | 5876 |  | 181 | commandWithSudo(): how every privileged trust-store command is wrapped | 3.7 | 1.5 | 0.693 |
| ns | 5961 |  | 85 | generateKey(): the key algorithm and sizes | 4.1 | 1.6 | 0.695 |
| walker |  | 5994 | 195 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.695 |
| walker |  | 6143 | 149 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 5, sub: 0, line: 106 } |  |  | 0.697 |
| ns | 6227 |  | 266 | fileNames(): the output filename convention | 4.2 | 1.6 | 0.679 |
| walker |  | 6293 | 150 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 6, sub: 0, line: 110 } |  |  | 0.679 |
| walker |  | 6444 | 151 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 4, sub: 0, line: 54 } |  |  | 0.679 |
| walker |  | 6453 | 9 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.685 |
| ns | 6558 |  | 331 | makeCert(): the leaf certificate template and its 2-year-3-month validity | 4.3 | 1.6 | 0.668 |
| walker |  | 6711 | 258 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 5, sub: 0, line: 77 } |  |  | 0.668 |
| walker |  | 6973 | 262 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 4, sub: 0, line: 55 } |  |  | 0.669 |
| ns | 6981 |  | 423 | makeCert(): SAN classification and extended key usages | 4.4 | 4.3 | 0.650 |
| walker |  | 7180 | 207 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.671 |
| ns | 7232 |  | 251 | makeCert(): signing, and the two write paths (PEM pair vs PKCS#12) with their file modes | 4.5 | 4.4 | 0.660 |
| walker |  | 7443 | 263 | Code::CodeKey { rung: Body, file: cert.go, decl: 6, sub: 0, line: 176 } |  |  | 0.688 |
| walker |  | 7487 | 44 | Code::CodeKey { rung: Body, file: main.go, decl: 15, sub: 0, line: 364 } |  |  | 0.688 |
| ns | 7515 |  | 283 | newCA(): the root certificate template | 4.6 | 1.6 | 0.673 |
| ns | 7759 |  | 244 | newCA(): writing rootCA-key.pem at 0400 and rootCA.pem at 0644 | 4.7 | 4.6 | 0.664 |
| walker |  | 7782 | 295 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.664 |
| ns | 7999 |  | 240 | loadCA(): create-on-demand, PEM parsing, and keyless mode | 4.8 | 1.6 | 0.653 |
| walker |  | 8030 | 248 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.653 |
| ns | 8212 |  | 213 | The identity strings: userAndHostname init, randomSerialNumber, caUniqueName | 4.9 | 1.6 | 0.658 |
| walker |  | 8269 | 239 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.658 |
| ns | 8476 |  | 264 | makeCertFromCSR(): the template fix-ups applied to a supplied CSR | 4.10 | 1.6 | 0.648 |
| walker |  | 8500 | 231 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 8605 |  | 129 | printHosts(): the certificate name list and the second-level wildcard warning | 4.11 | 1.6 | 0.652 |
| walker |  | 8710 | 210 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 3, sub: 0, line: 35 } |  |  | 0.653 |
| ns | 8856 |  | 251 | The macOS and Linux system-store install commands | 5.1 | 1.7 | 0.648 |
| walker |  | 8927 | 217 | Code::CodeKey { rung: Body, file: truststore_nss.go, decl: 7, sub: 0, line: 131 } |  |  | 0.649 |
| ns | 9088 |  | 232 | The Windows root-store install and uninstall, via crypt32 | 5.2 | 1.8 | 0.657 |
| walker |  | 9181 | 254 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.659 |
| walker |  | 9236 | 55 | Code::CodeKey { rung: Body, file: main.go, decl: 12, sub: 0, line: 336 } |  |  | 0.661 |
| ns | 9331 |  | 243 | installNSS(): the certutil invocation and its failure advice | 5.3 | 1.9 | 0.655 |
| ns | 9468 |  | 137 | installJava(): the keytool -importcert argument list | 5.4 | 1.9 | 0.659 |
| walker |  | 9599 | 363 | Code::CodeKey { rung: Body, file: truststore_linux.go, decl: 2, sub: 0, line: 27 } |  |  | 0.681 |
| ns | 9610 |  | 142 | forEachNSSProfile(): how Firefox/Chromium profiles are discovered and which DB format is used | 5.5 | 1.9 | 0.686 |
| ns | 9785 |  | 175 | The macOS and Linux uninstall commands, including the legacy filename cleanup | 5.6 | 5.1 | 0.689 |
| walker |  | 9812 | 213 | Code::CodeKey { rung: Body, file: truststore_windows.go, decl: 8, sub: 0, line: 91 } |  |  | 0.689 |
| ns | 9930 |  | 145 | go.mod: the direct and indirect dependency set | 6.1 | 1.3 | 0.686 |
| ns | 9974 |  | 44 | README: building from source with the version stamp | 6.2 |  | 0.687 |
| walker |  | 9981 | 169 | Code::CodeKey { rung: Body, file: truststore_java.go, decl: 2, sub: 0, line: 31 } |  |  | 0.695 |
