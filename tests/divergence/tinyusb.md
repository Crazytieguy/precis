Score(3000)=0.647 I=0.889 C=0.470 ns_rows≤3K=17/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.791/0.715/0.743/0.647/0.565/0.482/0.392

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 90 |  | 90 | Project identity: name + one-paragraph overview | 1.1 |  | 0.000 |
| walker |  | 107 | 107 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 114 | 7 | Fs::DirListing { dir: hw } |  |  | 0.000 |
| walker |  | 204 | 90 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 1.000 |
| ns | 205 |  | 115 | README 'Key Features' bullet list | 1.2 |  | 0.620 |
| walker |  | 221 | 17 | Fs::DirListing { dir: lib } |  |  | 0.620 |
| walker |  | 226 | 5 | Fs::DirListing { dir: lib/embedded-cli } |  |  | 0.620 |
| walker |  | 240 | 14 | Fs::DirListing { dir: lib/SEGGER_RTT } |  |  | 0.620 |
| walker |  | 249 | 9 | Fs::DirListing { dir: lib/SEGGER_RTT/Config } |  |  | 0.620 |
| walker |  | 271 | 22 | Fs::DirListing { dir: lib/rt-thread } |  |  | 0.620 |
| walker |  | 278 | 7 | Fs::DirListing { dir: lib/rt-thread/port } |  |  | 0.620 |
| ns | 312 |  | 107 | Complete repository root listing | 1.3 |  | 0.804 |
| walker |  | 313 | 35 | Fs::DirListing { dir: lib/SEGGER_RTT/RTT } |  |  | 0.804 |
| walker |  | 328 | 15 | Fs::DirListing { dir: hw/mcu } |  |  | 0.804 |
| walker |  | 365 | 37 | Fs::DirListing { dir: lib/networking } |  |  | 0.804 |
| walker |  | 400 | 35 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.804 |
| walker |  | 406 | 6 | Fs::DirListing { dir: .PVS-Studio } |  |  | 0.804 |
| walker |  | 412 | 6 | Fs::DirListing { dir: .claude } |  |  | 0.804 |
| walker |  | 422 | 10 | Fs::DirListing { dir: hw/mcu/dialog } |  |  | 0.804 |
| walker |  | 472 | 50 | Fs::DirListing { dir: src } |  |  | 0.811 |
| ns | 487 |  | 175 | README annotated top-level directory tree | 1.4 |  | 0.713 |
| walker |  | 492 | 20 | Fs::DirListing { dir: src/typec } |  |  | 0.717 |
| walker |  | 514 | 22 | Fs::DirListing { dir: src/device } |  |  | 0.721 |
| walker |  | 544 | 30 | Fs::DirListing { dir: src/host } |  |  | 0.730 |
| walker |  | 590 | 46 | Fs::DirListing { dir: src/class } |  |  | 0.734 |
| walker |  | 675 | 85 | Fs::DirListing { dir: src/portable } |  |  | 0.758 |
| walker |  | 728 | 53 | Fs::DirListing { dir: docs } |  |  | 0.758 |
| ns | 729 |  | 242 | Core stack file inventory: src/ and its non-fanout subdirectories | 1.5 |  | 0.632 |
| walker |  | 732 | 4 | Fs::DirListing { dir: docs/_static } |  |  | 0.632 |
| walker |  | 746 | 14 | Fs::DirListing { dir: docs/assets } |  |  | 0.632 |
| walker |  | 802 | 56 | Fs::DirListing { dir: src/common } |  |  | 0.721 |
| walker |  | 826 | 24 | Fs::DirListing { dir: docs/info } |  |  | 0.722 |
| ns | 879 |  | 150 | Architecture doc: layer/component overview | 1.6 |  | 0.688 |
| walker |  | 890 | 64 | Fs::DirListing { dir: src/osal } |  |  | 0.791 |
| walker |  | 899 | 9 | Fs::DirListing { dir: .circleci } |  |  | 0.791 |
| walker |  | 909 | 10 | Fs::DirListing { dir: test } |  |  | 0.791 |
| walker |  | 914 | 5 | Fs::DirListing { dir: .claude/commands } |  |  | 0.791 |
| walker |  | 925 | 11 | Fs::DirListing { dir: hw/mcu/sony/cxd56 } |  |  | 0.791 |
| walker |  | 962 | 37 | Fs::DirListing { dir: docs/reference } |  |  | 0.791 |
| walker |  | 974 | 12 | Fs::DirListing { dir: hw/mcu/bridgetek/ft9xx } |  |  | 0.791 |
| walker |  | 981 | 7 | Fs::DirListing { dir: .claude/skills } |  |  | 0.791 |
| walker |  | 986 | 5 | Fs::DirListing { dir: .claude/skills/code-size } |  |  | 0.791 |
| walker |  | 991 | 5 | Fs::DirListing { dir: .claude/skills/hil } |  |  | 0.791 |
| walker |  | 1009 | 18 | Fs::DirListing { dir: .github } |  |  | 0.791 |
| ns | 1010 |  | 131 | The two fan-out directories: src/class/* and src/portable/* (directory names) | 1.7 |  | 0.802 |
| walker |  | 1113 | 104 | Plaintext::DeclSurface { file: src/CMakeLists.txt } |  |  | 0.802 |
| walker |  | 1135 | 22 | Fs::DirListing { dir: .idea } |  |  | 0.802 |
| walker |  | 1205 | 70 | Fs::DirListing { dir: .github/workflows } |  |  | 0.802 |
| walker |  | 1229 | 24 | Fs::DirListing { dir: hw/mcu/dialog/da1469x } |  |  | 0.802 |
| walker |  | 1236 | 7 | Fs::DirListing { dir: hw/mcu/dialog/da1469x/include } |  |  | 0.802 |
| ns | 1237 |  | 227 | README host-stack and Power-Delivery capability lists | 1.8 |  | 0.750 |
| walker |  | 1241 | 5 | Fs::DirListing { dir: hw/mcu/dialog/da1469x/include/hal } |  |  | 0.750 |
| walker |  | 1252 | 11 | Fs::DirListing { dir: hw/mcu/sony/cxd56/tools } |  |  | 0.750 |
| ns | 1333 |  | 96 | README supported-CPU matrix: heading, elided table body, legend | 1.9 |  | 0.715 |
| walker |  | 1491 | 239 | Markdown::Section { file: README.rst, section_index: 3, keeps_default_concavity: true } |  |  | 0.715 |
| walker |  | 1525 | 34 | Fs::DirListing { dir: examples } |  |  | 0.715 |
| walker |  | 1536 | 11 | Fs::DirListing { dir: hw/mcu/dialog/da1469x/SDK_10.0.8.105/sdk/bsp } |  |  | 0.715 |
| walker |  | 1553 | 17 | Fs::DirListing { dir: examples/typec } |  |  | 0.715 |
| walker |  | 1571 | 18 | Fs::DirListing { dir: test/unit-test } |  |  | 0.716 |
| ns | 1579 |  | 246 | src/tusb.h: the stack-wide entry points | 2.1 |  | 0.677 |
| walker |  | 1588 | 17 | Fs::DirListing { dir: test/unit-test/test } |  |  | 0.677 |
| walker |  | 1594 | 6 | Fs::DirListing { dir: test/unit-test/test/support } |  |  | 0.677 |
| walker |  | 1602 | 8 | Fs::DirListing { dir: test/unit-test/test/device } |  |  | 0.677 |
| walker |  | 1608 | 6 | Fs::DirListing { dir: test/unit-test/test/device/usbd } |  |  | 0.677 |
| walker |  | 1615 | 7 | Fs::DirListing { dir: test/unit-test/test/device/msc } |  |  | 0.677 |
| ns | 1791 |  | 212 | src/device/usbd.h: complete roster of the tud_* application API (names only) | 2.2 |  | 0.640 |
| ns | 1952 |  | 161 | src/device/usbd.h: complete roster of tud_*_cb application callbacks | 2.3 |  | 0.617 |
| walker |  | 2001 | 386 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: true } |  |  | 0.743 |
| walker |  | 2022 | 21 | Fs::DirListing { dir: hw/mcu/dialog/da1469x/include/mcu } |  |  | 0.743 |
| walker |  | 2054 | 32 | Fs::DirListing { dir: hw/mcu/dialog/da1469x/src } |  |  | 0.743 |
| walker |  | 2078 | 24 | Fs::DirListing { dir: examples/typec/power_delivery } |  |  | 0.743 |
| walker |  | 2088 | 10 | Fs::DirListing { dir: examples/typec/power_delivery/src } |  |  | 0.743 |
| walker |  | 2124 | 36 | Fs::DirListing { dir: hw/mcu/sony/cxd56/mkspk } |  |  | 0.743 |
| ns | 2132 |  | 180 | src/host/usbh.h: host lifecycle API + application callback roster | 2.4 |  | 0.715 |
| walker |  | 2208 | 84 | Fs::DirListing { dir: tools } |  |  | 0.717 |
| walker |  | 2242 | 34 | Fs::DirListing { dir: examples/dual } |  |  | 0.717 |
| walker |  | 2266 | 24 | Fs::DirListing { dir: examples/dual/host_hid_to_device_cdc } |  |  | 0.717 |
| walker |  | 2282 | 16 | Fs::DirListing { dir: examples/dual/host_hid_to_device_cdc/src } |  |  | 0.717 |
| walker |  | 2306 | 24 | Fs::DirListing { dir: examples/dual/host_info_to_device_cdc } |  |  | 0.717 |
| ns | 2318 |  | 186 | src/host/usbh.h: per-device query API and endpoint/transfer API rosters | 2.5 |  | 0.690 |
| walker |  | 2328 | 22 | Fs::DirListing { dir: examples/dual/host_info_to_device_cdc/src } |  |  | 0.690 |
| walker |  | 2356 | 28 | Fs::DirListing { dir: examples/dual/dynamic_switch } |  |  | 0.690 |
| walker |  | 2378 | 22 | Fs::DirListing { dir: examples/dual/dynamic_switch/src } |  |  | 0.690 |
| walker |  | 2414 | 36 | Fs::DirListing { dir: test/hil } |  |  | 0.691 |
| walker |  | 2487 | 73 | Markdown::Section { file: README.rst, section_index: 6, keeps_default_concavity: false } |  |  | 0.696 |
| ns | 2557 |  | 239 | src/host/usbh.h: descriptor-fetch API, async and blocking variants | 2.6 |  | 0.668 |
| walker |  | 2564 | 77 | Markdown::Section { file: README.rst, section_index: 12, keeps_default_concavity: false } |  |  | 0.690 |
| walker |  | 2619 | 55 | Fs::DirListing { dir: test/fuzz } |  |  | 0.691 |
| walker |  | 2630 | 11 | Fs::DirListing { dir: test/fuzz/device } |  |  | 0.691 |
| walker |  | 2643 | 13 | Fs::DirListing { dir: test/fuzz/device/net } |  |  | 0.691 |
| walker |  | 2664 | 21 | Fs::DirListing { dir: test/fuzz/device/cdc } |  |  | 0.691 |
| walker |  | 2680 | 16 | Fs::DirListing { dir: test/fuzz/device/cdc/src } |  |  | 0.691 |
| walker |  | 2701 | 21 | Fs::DirListing { dir: test/fuzz/device/msc } |  |  | 0.691 |
| walker |  | 2717 | 16 | Fs::DirListing { dir: test/fuzz/device/msc/src } |  |  | 0.691 |
| walker |  | 2742 | 25 | Fs::DirListing { dir: test/fuzz/device/net/src } |  |  | 0.691 |
| walker |  | 2746 | 4 | Fs::DirListing { dir: test/fuzz/device/net/src/arch } |  |  | 0.691 |
| ns | 2825 |  | 268 | src/host/usbh.h: the tuh_xfer_t transfer descriptor | 2.7 |  | 0.662 |
| ns | 2940 |  | 115 | src/host/usbh.h: tuh_itf_info_t and tuh_bus_info_t | 2.8 |  | 0.646 |
| ns | 3110 |  | 170 | Runtime configure IDs for tud_configure() / tuh_configure() | 2.9 |  | 0.631 |
| ns | 3235 |  | 125 | src/tusb_option.h: stack version macros | 3.1 |  | 0.624 |
| ns | 3391 |  | 156 | src/tusb_option.h: complete section map (headings only) | 3.2 |  | 0.610 |
| walker |  | 3407 | 661 | Plaintext::Whole { file: src/CMakeLists.txt } |  |  | 0.611 |
| walker |  | 3472 | 65 | Fs::DirListing { dir: examples/host } |  |  | 0.611 |
| ns | 3486 |  | 95 | src/tusb_option.h: how tusb_config.h is pulled in | 3.3 |  | 0.602 |
| walker |  | 3500 | 28 | Fs::DirListing { dir: examples/host/bare_api } |  |  | 0.602 |
| walker |  | 3510 | 10 | Fs::DirListing { dir: examples/host/bare_api/src } |  |  | 0.602 |
| walker |  | 3538 | 28 | Fs::DirListing { dir: examples/host/cdc_msc_hid } |  |  | 0.602 |
| walker |  | 3566 | 28 | Fs::DirListing { dir: examples/host/cdc_msc_hid_freertos } |  |  | 0.602 |
| walker |  | 3594 | 28 | Fs::DirListing { dir: examples/host/device_info } |  |  | 0.602 |
| walker |  | 3610 | 16 | Fs::DirListing { dir: examples/host/device_info/src } |  |  | 0.602 |
| walker |  | 3638 | 28 | Fs::DirListing { dir: examples/host/hid_controller } |  |  | 0.602 |
| walker |  | 3657 | 19 | Fs::DirListing { dir: examples/host/hid_controller/src } |  |  | 0.602 |
| walker |  | 3685 | 28 | Fs::DirListing { dir: examples/host/midi_rx } |  |  | 0.602 |
| walker |  | 3695 | 10 | Fs::DirListing { dir: examples/host/midi_rx/src } |  |  | 0.602 |
| walker |  | 3726 | 31 | Fs::DirListing { dir: examples/host/cdc_msc_hid/src } |  |  | 0.602 |
| walker |  | 3758 | 32 | Fs::DirListing { dir: examples/host/msc_file_explorer } |  |  | 0.602 |
| walker |  | 3785 | 27 | Fs::DirListing { dir: examples/host/msc_file_explorer/src } |  |  | 0.602 |
| walker |  | 3817 | 32 | Fs::DirListing { dir: examples/host/msc_file_explorer_freertos } |  |  | 0.602 |
| ns | 3818 |  | 332 | src/tusb_option.h: every device class-driver enable macro with its default | 3.4 |  | 0.584 |
| walker |  | 3850 | 33 | Fs::DirListing { dir: examples/host/msc_file_explorer_freertos/src } |  |  | 0.584 |
| walker |  | 3887 | 37 | Fs::DirListing { dir: examples/host/cdc_msc_hid_freertos/src } |  |  | 0.584 |
| ns | 4148 |  | 330 | src/tusb_option.h: every host class-driver enable macro, with elided VID/PID tables | 3.5 |  | 0.565 |
| walker |  | 4376 | 489 | Fs::DirListing { dir: hw/bsp } |  |  | 0.565 |
| walker |  | 4387 | 11 | Fs::DirListing { dir: hw/bsp/espressif } |  |  | 0.565 |
| walker |  | 4398 | 11 | Fs::DirListing { dir: hw/bsp/pic32mz } |  |  | 0.565 |
| walker |  | 4407 | 9 | Fs::DirListing { dir: hw/bsp/espressif/components } |  |  | 0.565 |
| walker |  | 4423 | 16 | Fs::DirListing { dir: hw/bsp/broadcom_32bit } |  |  | 0.565 |
| walker |  | 4439 | 16 | Fs::DirListing { dir: hw/bsp/broadcom_64bit } |  |  | 0.565 |
| walker |  | 4455 | 16 | Fs::DirListing { dir: hw/bsp/ft9xx } |  |  | 0.565 |
| walker |  | 4471 | 16 | Fs::DirListing { dir: hw/bsp/hpmicro } |  |  | 0.565 |
| walker |  | 4487 | 16 | Fs::DirListing { dir: hw/bsp/same7x } |  |  | 0.565 |
| walker |  | 4493 | 6 | Fs::DirListing { dir: hw/bsp/espressif/components/tinyusb_src } |  |  | 0.565 |
| ns | 4497 |  | 349 | src/tusb_option.h: OS selector and roothub mode/speed constants | 3.6 |  | 0.551 |
| walker |  | 4513 | 20 | Fs::DirListing { dir: hw/bsp/f1c100s } |  |  | 0.551 |
| walker |  | 4525 | 12 | Fs::DirListing { dir: hw/bsp/broadcom_64bit/boards } |  |  | 0.551 |
| walker |  | 4547 | 22 | Fs::DirListing { dir: hw/bsp/cxd56 } |  |  | 0.551 |
| walker |  | 4554 | 7 | Fs::DirListing { dir: hw/bsp/cxd56/FreeRTOSConfig } |  |  | 0.551 |
| walker |  | 4576 | 22 | Fs::DirListing { dir: hw/bsp/efm32 } |  |  | 0.551 |
| walker |  | 4583 | 7 | Fs::DirListing { dir: hw/bsp/efm32/FreeRTOSConfig } |  |  | 0.551 |
| walker |  | 4605 | 22 | Fs::DirListing { dir: hw/bsp/kinetis_k } |  |  | 0.551 |
| walker |  | 4612 | 7 | Fs::DirListing { dir: hw/bsp/kinetis_k/FreeRTOSConfig } |  |  | 0.551 |
| walker |  | 4634 | 22 | Fs::DirListing { dir: hw/bsp/kinetis_k32l } |  |  | 0.551 |
| walker |  | 4641 | 7 | Fs::DirListing { dir: hw/bsp/kinetis_k32l/FreeRTOSConfig } |  |  | 0.551 |
| walker |  | 4663 | 22 | Fs::DirListing { dir: hw/bsp/lpc11 } |  |  | 0.551 |
| walker |  | 4670 | 7 | Fs::DirListing { dir: hw/bsp/lpc11/FreeRTOSConfig } |  |  | 0.551 |
| walker |  | 4692 | 22 | Fs::DirListing { dir: hw/bsp/lpc13 } |  |  | 0.551 |
| ns | 4693 |  | 196 | src/tusb_option.h: common (CFG_TUSB_*) option defaults | 3.7 | 3.2 | 0.543 |
| walker |  | 4699 | 7 | Fs::DirListing { dir: hw/bsp/lpc13/FreeRTOSConfig } |  |  | 0.543 |
| walker |  | 4721 | 22 | Fs::DirListing { dir: hw/bsp/lpc15 } |  |  | 0.543 |
| walker |  | 4728 | 7 | Fs::DirListing { dir: hw/bsp/lpc15/FreeRTOSConfig } |  |  | 0.543 |
| walker |  | 4750 | 22 | Fs::DirListing { dir: hw/bsp/lpc17 } |  |  | 0.543 |
| walker |  | 4757 | 7 | Fs::DirListing { dir: hw/bsp/lpc17/FreeRTOSConfig } |  |  | 0.543 |
| walker |  | 4779 | 22 | Fs::DirListing { dir: hw/bsp/lpc18 } |  |  | 0.543 |
| walker |  | 4786 | 7 | Fs::DirListing { dir: hw/bsp/lpc18/FreeRTOSConfig } |  |  | 0.543 |
| walker |  | 4808 | 22 | Fs::DirListing { dir: hw/bsp/lpc40 } |  |  | 0.543 |
| walker |  | 4815 | 7 | Fs::DirListing { dir: hw/bsp/lpc40/FreeRTOSConfig } |  |  | 0.543 |
| walker |  | 4837 | 22 | Fs::DirListing { dir: hw/bsp/lpc43 } |  |  | 0.543 |
| walker |  | 4844 | 7 | Fs::DirListing { dir: hw/bsp/lpc43/FreeRTOSConfig } |  |  | 0.543 |
| ns | 4865 |  | 172 | src/tusb_option.h: device-side sizing and behaviour defaults | 3.8 | 3.2 | 0.536 |
| walker |  | 4866 | 22 | Fs::DirListing { dir: hw/bsp/lpc51 } |  |  | 0.536 |
| walker |  | 4873 | 7 | Fs::DirListing { dir: hw/bsp/lpc51/FreeRTOSConfig } |  |  | 0.536 |
| walker |  | 4895 | 22 | Fs::DirListing { dir: hw/bsp/lpc55 } |  |  | 0.536 |
| walker |  | 4902 | 7 | Fs::DirListing { dir: hw/bsp/lpc55/FreeRTOSConfig } |  |  | 0.536 |
| ns | 4922 |  | 57 | src/tusb_option.h: host-side sizing defaults | 3.9 | 3.2 | 0.533 |
| walker |  | 4924 | 22 | Fs::DirListing { dir: hw/bsp/mm32 } |  |  | 0.533 |
| walker |  | 4931 | 7 | Fs::DirListing { dir: hw/bsp/mm32/FreeRTOSConfig } |  |  | 0.533 |
| walker |  | 4953 | 22 | Fs::DirListing { dir: hw/bsp/msp430 } |  |  | 0.533 |
| walker |  | 4960 | 7 | Fs::DirListing { dir: hw/bsp/msp430/FreeRTOSConfig } |  |  | 0.533 |
| walker |  | 4982 | 22 | Fs::DirListing { dir: hw/bsp/msp432e4 } |  |  | 0.533 |
| walker |  | 4989 | 7 | Fs::DirListing { dir: hw/bsp/msp432e4/FreeRTOSConfig } |  |  | 0.533 |
| walker |  | 5011 | 22 | Fs::DirListing { dir: hw/bsp/nuc100_120 } |  |  | 0.533 |
| walker |  | 5018 | 7 | Fs::DirListing { dir: hw/bsp/nuc100_120/FreeRTOSConfig } |  |  | 0.533 |
| ns | 5032 |  | 110 | src/tusb_option.h: per-controller (USBIP) configuration sub-sections | 3.10 |  | 0.527 |
| walker |  | 5040 | 22 | Fs::DirListing { dir: hw/bsp/nuc121_125 } |  |  | 0.527 |
| walker |  | 5047 | 7 | Fs::DirListing { dir: hw/bsp/nuc121_125/FreeRTOSConfig } |  |  | 0.527 |
| walker |  | 5069 | 22 | Fs::DirListing { dir: hw/bsp/nuc126 } |  |  | 0.527 |
| walker |  | 5076 | 7 | Fs::DirListing { dir: hw/bsp/nuc126/FreeRTOSConfig } |  |  | 0.527 |
| walker |  | 5098 | 22 | Fs::DirListing { dir: hw/bsp/nuc505 } |  |  | 0.527 |
| walker |  | 5105 | 7 | Fs::DirListing { dir: hw/bsp/nuc505/FreeRTOSConfig } |  |  | 0.527 |
| ns | 5126 |  | 94 | src/tusb_option.h: TypeC enable and compile-time configuration validation | 3.11 |  | 0.522 |
| walker |  | 5127 | 22 | Fs::DirListing { dir: hw/bsp/rw61x } |  |  | 0.522 |
| walker |  | 5134 | 7 | Fs::DirListing { dir: hw/bsp/rw61x/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5156 | 22 | Fs::DirListing { dir: hw/bsp/rx } |  |  | 0.522 |
| walker |  | 5163 | 7 | Fs::DirListing { dir: hw/bsp/rx/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5175 | 12 | Fs::DirListing { dir: hw/bsp/rx/boards } |  |  | 0.522 |
| walker |  | 5197 | 22 | Fs::DirListing { dir: hw/bsp/samd11 } |  |  | 0.522 |
| walker |  | 5204 | 7 | Fs::DirListing { dir: hw/bsp/samd11/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5226 | 22 | Fs::DirListing { dir: hw/bsp/samd2x_l2x } |  |  | 0.522 |
| walker |  | 5233 | 7 | Fs::DirListing { dir: hw/bsp/samd2x_l2x/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5255 | 22 | Fs::DirListing { dir: hw/bsp/samd5x_e5x } |  |  | 0.522 |
| walker |  | 5262 | 7 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5284 | 22 | Fs::DirListing { dir: hw/bsp/tm4c } |  |  | 0.522 |
| walker |  | 5291 | 7 | Fs::DirListing { dir: hw/bsp/tm4c/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5313 | 22 | Fs::DirListing { dir: hw/bsp/xmc4000 } |  |  | 0.522 |
| walker |  | 5320 | 7 | Fs::DirListing { dir: hw/bsp/xmc4000/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5333 | 13 | Fs::DirListing { dir: hw/bsp/kinetis_k/boards } |  |  | 0.522 |
| walker |  | 5347 | 14 | Fs::DirListing { dir: hw/bsp/lpc17/boards } |  |  | 0.522 |
| walker |  | 5361 | 14 | Fs::DirListing { dir: hw/bsp/lpc43/boards } |  |  | 0.522 |
| walker |  | 5375 | 14 | Fs::DirListing { dir: hw/bsp/same7x/boards } |  |  | 0.522 |
| walker |  | 5400 | 25 | Fs::DirListing { dir: hw/bsp/gd32vf103 } |  |  | 0.522 |
| walker |  | 5425 | 25 | Fs::DirListing { dir: hw/bsp/kinetis_kl } |  |  | 0.522 |
| walker |  | 5432 | 7 | Fs::DirListing { dir: hw/bsp/kinetis_kl/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5457 | 25 | Fs::DirListing { dir: hw/bsp/lpc54 } |  |  | 0.522 |
| walker |  | 5464 | 7 | Fs::DirListing { dir: hw/bsp/lpc54/FreeRTOSConfig } |  |  | 0.522 |
| walker |  | 5479 | 15 | Fs::DirListing { dir: hw/bsp/lpc18/boards } |  |  | 0.511 |
| ns | 5479 |  | 353 | src/device/usbd_pvt.h: the device class-driver vtable | 4.1 |  | 0.511 |
| walker |  | 5494 | 15 | Fs::DirListing { dir: hw/bsp/samd11/boards } |  |  | 0.511 |
| walker |  | 5510 | 16 | Fs::DirListing { dir: hw/bsp/nuc121_125/boards } |  |  | 0.511 |
| walker |  | 5526 | 16 | Fs::DirListing { dir: hw/bsp/pic32mz/boards } |  |  | 0.511 |
| walker |  | 5542 | 16 | Fs::DirListing { dir: hw/bsp/xmc4000/boards } |  |  | 0.511 |
| walker |  | 5570 | 28 | Fs::DirListing { dir: hw/bsp/imxrt } |  |  | 0.511 |
| walker |  | 5577 | 7 | Fs::DirListing { dir: hw/bsp/imxrt/FreeRTOSConfig } |  |  | 0.511 |
| walker |  | 5606 | 29 | Fs::DirListing { dir: hw/bsp/maxim } |  |  | 0.511 |
| walker |  | 5613 | 7 | Fs::DirListing { dir: hw/bsp/maxim/FreeRTOSConfig } |  |  | 0.511 |
| walker |  | 5643 | 30 | Fs::DirListing { dir: hw/bsp/nrf } |  |  | 0.511 |
| walker |  | 5650 | 7 | Fs::DirListing { dir: hw/bsp/nrf/FreeRTOSConfig } |  |  | 0.511 |
| walker |  | 5668 | 18 | Fs::DirListing { dir: hw/bsp/lpc11/boards } |  |  | 0.511 |
| walker |  | 5686 | 18 | Fs::DirListing { dir: hw/bsp/tm4c/boards } |  |  | 0.511 |
| walker |  | 5717 | 31 | Fs::DirListing { dir: hw/bsp/mcx } |  |  | 0.511 |
| walker |  | 5724 | 7 | Fs::DirListing { dir: hw/bsp/mcx/FreeRTOSConfig } |  |  | 0.511 |
| walker |  | 5755 | 31 | Fs::DirListing { dir: hw/bsp/stm32wb } |  |  | 0.511 |
| walker |  | 5762 | 7 | Fs::DirListing { dir: hw/bsp/stm32wb/FreeRTOSConfig } |  |  | 0.511 |
| ns | 5788 |  | 309 | src/host/usbh_pvt.h: the host class-driver vtable and USBH hooks | 4.2 |  | 0.500 |
| walker |  | 5794 | 32 | Fs::DirListing { dir: hw/bsp/stm32c0 } |  |  | 0.500 |
| walker |  | 5801 | 7 | Fs::DirListing { dir: hw/bsp/stm32c0/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 5833 | 32 | Fs::DirListing { dir: hw/bsp/stm32f0 } |  |  | 0.500 |
| walker |  | 5840 | 7 | Fs::DirListing { dir: hw/bsp/stm32f0/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 5872 | 32 | Fs::DirListing { dir: hw/bsp/stm32f1 } |  |  | 0.500 |
| walker |  | 5879 | 7 | Fs::DirListing { dir: hw/bsp/stm32f1/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 5911 | 32 | Fs::DirListing { dir: hw/bsp/stm32f2 } |  |  | 0.500 |
| walker |  | 5918 | 7 | Fs::DirListing { dir: hw/bsp/stm32f2/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 5950 | 32 | Fs::DirListing { dir: hw/bsp/stm32f3 } |  |  | 0.500 |
| walker |  | 5957 | 7 | Fs::DirListing { dir: hw/bsp/stm32f3/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 5989 | 32 | Fs::DirListing { dir: hw/bsp/stm32f4 } |  |  | 0.500 |
| walker |  | 5996 | 7 | Fs::DirListing { dir: hw/bsp/stm32f4/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 6028 | 32 | Fs::DirListing { dir: hw/bsp/stm32f7 } |  |  | 0.500 |
| walker |  | 6035 | 7 | Fs::DirListing { dir: hw/bsp/stm32f7/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 6067 | 32 | Fs::DirListing { dir: hw/bsp/stm32g0 } |  |  | 0.500 |
| walker |  | 6074 | 7 | Fs::DirListing { dir: hw/bsp/stm32g0/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 6106 | 32 | Fs::DirListing { dir: hw/bsp/stm32g4 } |  |  | 0.500 |
| walker |  | 6113 | 7 | Fs::DirListing { dir: hw/bsp/stm32g4/FreeRTOSConfig } |  |  | 0.500 |
| walker |  | 6145 | 32 | Fs::DirListing { dir: hw/bsp/stm32l0 } |  |  | 0.500 |
| walker |  | 6152 | 7 | Fs::DirListing { dir: hw/bsp/stm32l0/FreeRTOSConfig } |  |  | 0.500 |
| ns | 6164 |  | 376 | src/device/dcd.h: complete device-controller porting contract (names only) | 4.3 |  | 0.482 |
| walker |  | 6169 | 17 | Fs::DirListing { dir: hw/bsp/stm32l0/boards } |  |  | 0.482 |
| walker |  | 6201 | 32 | Fs::DirListing { dir: hw/bsp/stm32l4 } |  |  | 0.482 |
| walker |  | 6208 | 7 | Fs::DirListing { dir: hw/bsp/stm32l4/FreeRTOSConfig } |  |  | 0.482 |
| walker |  | 6242 | 34 | Fs::DirListing { dir: hw/bsp/stm32wba } |  |  | 0.482 |
| walker |  | 6249 | 7 | Fs::DirListing { dir: hw/bsp/stm32wba/FreeRTOSConfig } |  |  | 0.482 |
| walker |  | 6260 | 11 | Fs::DirListing { dir: hw/bsp/stm32wba/linker } |  |  | 0.482 |
| walker |  | 6280 | 20 | Fs::DirListing { dir: hw/bsp/kinetis_kl/gcc } |  |  | 0.482 |
| walker |  | 6293 | 13 | Fs::DirListing { dir: hw/bsp/broadcom_64bit/boards/raspberrypi_cm4 } |  |  | 0.482 |
| walker |  | 6306 | 13 | Fs::DirListing { dir: hw/bsp/broadcom_64bit/boards/raspberrypi_zero2 } |  |  | 0.482 |
| walker |  | 6319 | 13 | Fs::DirListing { dir: hw/bsp/xmc4000/boards/xmc4500_relax } |  |  | 0.482 |
| walker |  | 6332 | 13 | Fs::DirListing { dir: hw/bsp/xmc4000/boards/xmc4700_relax } |  |  | 0.482 |
| walker |  | 6367 | 35 | Fs::DirListing { dir: hw/bsp/stm32h5 } |  |  | 0.482 |
| walker |  | 6374 | 7 | Fs::DirListing { dir: hw/bsp/stm32h5/FreeRTOSConfig } |  |  | 0.482 |
| walker |  | 6409 | 35 | Fs::DirListing { dir: hw/bsp/stm32h7 } |  |  | 0.482 |
| walker |  | 6416 | 7 | Fs::DirListing { dir: hw/bsp/stm32h7/FreeRTOSConfig } |  |  | 0.482 |
| walker |  | 6451 | 35 | Fs::DirListing { dir: hw/bsp/stm32u0 } |  |  | 0.482 |
| walker |  | 6458 | 7 | Fs::DirListing { dir: hw/bsp/stm32u0/FreeRTOSConfig } |  |  | 0.482 |
| ns | 6463 |  | 299 | src/host/hcd.h: complete host-controller porting contract (names only) | 4.4 |  | 0.469 |
| walker |  | 6475 | 17 | Fs::DirListing { dir: hw/bsp/stm32u0/boards } |  |  | 0.469 |
| walker |  | 6494 | 19 | Fs::DirListing { dir: hw/bsp/stm32u0/linker } |  |  | 0.469 |
| walker |  | 6507 | 13 | Fs::DirListing { dir: hw/bsp/stm32u0/boards/stm32u083cdk } |  |  | 0.469 |
| walker |  | 6542 | 35 | Fs::DirListing { dir: hw/bsp/stm32u5 } |  |  | 0.469 |
| walker |  | 6549 | 7 | Fs::DirListing { dir: hw/bsp/stm32u5/FreeRTOSConfig } |  |  | 0.469 |
| walker |  | 6570 | 21 | Fs::DirListing { dir: hw/bsp/mm32/boards } |  |  | 0.469 |
| walker |  | 6606 | 36 | Fs::DirListing { dir: hw/bsp/samg } |  |  | 0.469 |
| walker |  | 6613 | 7 | Fs::DirListing { dir: hw/bsp/samg/FreeRTOSConfig } |  |  | 0.469 |
| walker |  | 6649 | 36 | Fs::DirListing { dir: hw/bsp/stm32h7rs } |  |  | 0.469 |
| walker |  | 6656 | 7 | Fs::DirListing { dir: hw/bsp/stm32h7rs/FreeRTOSConfig } |  |  | 0.469 |
| walker |  | 6667 | 11 | Fs::DirListing { dir: hw/bsp/stm32h7rs/linker } |  |  | 0.469 |
| walker |  | 6705 | 38 | Fs::DirListing { dir: hw/bsp/ra } |  |  | 0.469 |
| walker |  | 6712 | 7 | Fs::DirListing { dir: hw/bsp/ra/FreeRTOSConfig } |  |  | 0.469 |
| walker |  | 6752 | 40 | Fs::DirListing { dir: hw/bsp/da1469x } |  |  | 0.469 |
| walker |  | 6759 | 7 | Fs::DirListing { dir: hw/bsp/da1469x/FreeRTOSConfig } |  |  | 0.469 |
| walker |  | 6766 | 7 | Fs::DirListing { dir: hw/bsp/da1469x/linker } |  |  | 0.469 |
| walker |  | 6783 | 17 | Fs::DirListing { dir: hw/bsp/da1469x/boards } |  |  | 0.469 |
| walker |  | 6807 | 24 | Fs::DirListing { dir: hw/bsp/kinetis_k32l/boards } |  |  | 0.469 |
| walker |  | 6831 | 24 | Fs::DirListing { dir: hw/bsp/lpc54/boards } |  |  | 0.469 |
| walker |  | 6844 | 13 | Fs::DirListing { dir: hw/bsp/lpc54/boards/lpcxpresso54114 } |  |  | 0.469 |
| walker |  | 6857 | 13 | Fs::DirListing { dir: hw/bsp/lpc54/boards/lpcxpresso54608 } |  |  | 0.469 |
| walker |  | 6870 | 13 | Fs::DirListing { dir: hw/bsp/lpc54/boards/lpcxpresso54628 } |  |  | 0.469 |
| ns | 6880 |  | 417 | src/osal/osal.h: the OSAL porting contract | 4.5 |  | 0.457 |
| walker |  | 6895 | 25 | Fs::DirListing { dir: hw/bsp/stm32f0/boards } |  |  | 0.457 |
| walker |  | 6911 | 16 | Fs::DirListing { dir: hw/bsp/cxd56/boards/spresense } |  |  | 0.457 |
| walker |  | 6927 | 16 | Fs::DirListing { dir: hw/bsp/mcx/drivers/spc } |  |  | 0.457 |
| walker |  | 6970 | 43 | Fs::DirListing { dir: hw/bsp/rp2040 } |  |  | 0.457 |
| walker |  | 6977 | 7 | Fs::DirListing { dir: hw/bsp/rp2040/FreeRTOSConfig } |  |  | 0.457 |
| walker |  | 6994 | 17 | Fs::DirListing { dir: hw/bsp/da1469x/boards/da14695_dk_usb } |  |  | 0.457 |
| walker |  | 6999 | 5 | Fs::DirListing { dir: hw/bsp/da1469x/boards/da14695_dk_usb/syscfg } |  |  | 0.457 |
| walker |  | 7016 | 17 | Fs::DirListing { dir: hw/bsp/da1469x/boards/da1469x_dk_pro } |  |  | 0.457 |
| walker |  | 7021 | 5 | Fs::DirListing { dir: hw/bsp/da1469x/boards/da1469x_dk_pro/syscfg } |  |  | 0.457 |
| ns | 7023 |  | 143 | hw/bsp/board_api.h: complete board porting API (names only) | 4.6 |  | 0.451 |
| walker |  | 7038 | 17 | Fs::DirListing { dir: hw/bsp/ft9xx/boards/mm900evxb } |  |  | 0.451 |
| walker |  | 7055 | 17 | Fs::DirListing { dir: hw/bsp/mm32/boards/mm32f327x_mb39 } |  |  | 0.451 |
| walker |  | 7072 | 17 | Fs::DirListing { dir: hw/bsp/mm32/boards/mm32f327x_pitaya_lite } |  |  | 0.451 |
| walker |  | 7089 | 17 | Fs::DirListing { dir: hw/bsp/pic32mz/boards/olimex_emz64 } |  |  | 0.451 |
| walker |  | 7106 | 17 | Fs::DirListing { dir: hw/bsp/pic32mz/boards/olimex_hmz144 } |  |  | 0.451 |
| walker |  | 7133 | 27 | Fs::DirListing { dir: hw/bsp/stm32f1/boards } |  |  | 0.451 |
| walker |  | 7146 | 13 | Fs::DirListing { dir: hw/bsp/stm32f1/boards/stm32f103ze_iar } |  |  | 0.451 |
| walker |  | 7173 | 27 | Fs::DirListing { dir: hw/bsp/stm32g4/boards } |  |  | 0.451 |
| walker |  | 7200 | 27 | Fs::DirListing { dir: hw/bsp/stm32h5/boards } |  |  | 0.451 |
| walker |  | 7213 | 13 | Fs::DirListing { dir: hw/bsp/stm32h5/boards/stm32h503nucleo } |  |  | 0.451 |
| walker |  | 7226 | 13 | Fs::DirListing { dir: hw/bsp/stm32h5/boards/stm32h573i_dk } |  |  | 0.451 |
| walker |  | 7274 | 48 | Fs::DirListing { dir: hw/bsp/fomu } |  |  | 0.451 |
| walker |  | 7285 | 11 | Fs::DirListing { dir: hw/bsp/fomu/include } |  |  | 0.442 |
| ns | 7285 |  | 262 | Endpoint API a class driver is allowed to call | 4.7 |  | 0.442 |
| walker |  | 7289 | 4 | Fs::DirListing { dir: hw/bsp/fomu/include/hw } |  |  | 0.442 |
| walker |  | 7304 | 15 | Fs::DirListing { dir: hw/bsp/fomu/boards/fomu } |  |  | 0.442 |
| walker |  | 7322 | 18 | Fs::DirListing { dir: hw/bsp/efm32/boards/sltb009a } |  |  | 0.442 |
| walker |  | 7340 | 18 | Fs::DirListing { dir: hw/bsp/f1c100s/boards/f1c100s } |  |  | 0.442 |
| walker |  | 7358 | 18 | Fs::DirListing { dir: hw/bsp/stm32h5/boards/stm32h563nucleo } |  |  | 0.442 |
| walker |  | 7369 | 11 | Fs::DirListing { dir: hw/bsp/stm32h5/boards/stm32h563nucleo/cubemx } |  |  | 0.442 |
| walker |  | 7387 | 18 | Fs::DirListing { dir: hw/bsp/stm32u0/boards/stm32u083nucleo } |  |  | 0.442 |
| walker |  | 7394 | 7 | Fs::DirListing { dir: hw/bsp/stm32u0/boards/stm32u083nucleo/cubemx } |  |  | 0.442 |
| walker |  | 7423 | 29 | Fs::DirListing { dir: hw/bsp/mcx/boards } |  |  | 0.442 |
| walker |  | 7442 | 19 | Fs::DirListing { dir: hw/bsp/broadcom_32bit/boards/raspberrypi_zero } |  |  | 0.442 |
| walker |  | 7461 | 19 | Fs::DirListing { dir: hw/bsp/nuc121_125/boards/nutiny_sdk_nuc121 } |  |  | 0.442 |
| walker |  | 7480 | 19 | Fs::DirListing { dir: hw/bsp/nuc121_125/boards/nutiny_sdk_nuc125 } |  |  | 0.442 |
| walker |  | 7510 | 30 | Fs::DirListing { dir: hw/bsp/lpc55/boards } |  |  | 0.442 |
| walker |  | 7540 | 30 | Fs::DirListing { dir: hw/bsp/maxim/linker } |  |  | 0.442 |
| walker |  | 7560 | 20 | Fs::DirListing { dir: hw/bsp/gd32vf103/boards/sipeed_longan_nano } |  |  | 0.442 |
| walker |  | 7580 | 20 | Fs::DirListing { dir: hw/bsp/lpc17/boards/lpcxpresso1769 } |  |  | 0.442 |
| walker |  | 7600 | 20 | Fs::DirListing { dir: hw/bsp/lpc17/boards/mbed1768 } |  |  | 0.442 |
| ns | 7602 |  | 317 | Complete file inventory of every USB class driver | 5.1 |  | 0.419 |
| walker |  | 7620 | 20 | Fs::DirListing { dir: hw/bsp/lpc18/boards/lpcxpresso18s37 } |  |  | 0.419 |
| walker |  | 7640 | 20 | Fs::DirListing { dir: hw/bsp/lpc43/boards/lpcxpresso43s67 } |  |  | 0.419 |
| walker |  | 7660 | 20 | Fs::DirListing { dir: hw/bsp/lpc51/boards/lpcxpresso51u68 } |  |  | 0.419 |
| walker |  | 7680 | 20 | Fs::DirListing { dir: hw/bsp/msp432e4/boards/msp_exp432e401y } |  |  | 0.419 |
| walker |  | 7712 | 32 | Fs::DirListing { dir: hw/bsp/lpc54/iar } |  |  | 0.419 |
| walker |  | 7768 | 56 | Fs::DirListing { dir: hw/bsp/stm32n6 } |  |  | 0.419 |
| walker |  | 7775 | 7 | Fs::DirListing { dir: hw/bsp/stm32n6/FreeRTOSConfig } |  |  | 0.419 |
| walker |  | 7792 | 17 | Fs::DirListing { dir: hw/bsp/stm32n6/boards } |  |  | 0.419 |
| walker |  | 7813 | 21 | Fs::DirListing { dir: hw/bsp/lpc11/boards/lpcxpresso11u37 } |  |  | 0.419 |
| walker |  | 7834 | 21 | Fs::DirListing { dir: hw/bsp/lpc11/boards/lpcxpresso11u68 } |  |  | 0.419 |
| walker |  | 7855 | 21 | Fs::DirListing { dir: hw/bsp/msp430/boards/msp_exp430f5529lp } |  |  | 0.419 |
| ns | 7860 |  | 258 | src/device/usbd.c: the built-in device class-driver table | 5.2 |  | 0.413 |
| walker |  | 7876 | 21 | Fs::DirListing { dir: hw/bsp/samd11/boards/cynthion_d11 } |  |  | 0.413 |
| walker |  | 7897 | 21 | Fs::DirListing { dir: hw/bsp/stm32wba/boards/stm32wba_nucleo } |  |  | 0.413 |
| walker |  | 7930 | 33 | Fs::DirListing { dir: hw/bsp/nrf/nrfx_config } |  |  | 0.413 |
| walker |  | 7953 | 23 | Fs::DirListing { dir: hw/bsp/kinetis_k32l/boards/frdm_k32l2a4s } |  |  | 0.413 |
| walker |  | 7976 | 23 | Fs::DirListing { dir: hw/bsp/kinetis_k32l/boards/frdm_k32l2b } |  |  | 0.413 |
| ns | 7983 |  | 123 | src/host/usbh.c: the built-in host class-driver table | 5.3 |  | 0.411 |
| walker |  | 7999 | 23 | Fs::DirListing { dir: hw/bsp/lpc18/boards/mcb1800 } |  |  | 0.411 |
| walker |  | 8007 | 8 | Fs::DirListing { dir: hw/bsp/lpc18/boards/mcb1800/ozone } |  |  | 0.411 |
| walker |  | 8030 | 23 | Fs::DirListing { dir: hw/bsp/lpc55/boards/mcu_link } |  |  | 0.411 |
| walker |  | 8053 | 23 | Fs::DirListing { dir: hw/bsp/samd11/boards/samd11_xplained } |  |  | 0.411 |
| walker |  | 8076 | 23 | Fs::DirListing { dir: hw/bsp/stm32f0/boards/stm32f070rbnucleo } |  |  | 0.411 |
| walker |  | 8099 | 23 | Fs::DirListing { dir: hw/bsp/stm32g4/boards/stm32g474nucleo } |  |  | 0.411 |
| walker |  | 8122 | 23 | Fs::DirListing { dir: hw/bsp/stm32g4/boards/stm32g491nucleo } |  |  | 0.411 |
| walker |  | 8145 | 23 | Fs::DirListing { dir: hw/bsp/stm32h7rs/boards/stm32h7s3nucleo } |  |  | 0.411 |
| walker |  | 8207 | 62 | Fs::DirListing { dir: hw/bsp/at32f413 } |  |  | 0.411 |
| walker |  | 8214 | 7 | Fs::DirListing { dir: hw/bsp/at32f413/FreeRTOSConfig } |  |  | 0.411 |
| walker |  | 8232 | 18 | Fs::DirListing { dir: hw/bsp/at32f413/boards/at_start_f413 } |  |  | 0.411 |
| walker |  | 8294 | 62 | Fs::DirListing { dir: hw/bsp/at32f415 } |  |  | 0.411 |
| walker |  | 8301 | 7 | Fs::DirListing { dir: hw/bsp/at32f415/FreeRTOSConfig } |  |  | 0.411 |
| walker |  | 8319 | 18 | Fs::DirListing { dir: hw/bsp/at32f415/boards/at_start_f415 } |  |  | 0.411 |
| ns | 8321 |  | 338 | src/device/usbd.h: complete roster of descriptor template macros | 5.4 |  | 0.402 |
| walker |  | 8381 | 62 | Fs::DirListing { dir: hw/bsp/at32f423 } |  |  | 0.402 |
| walker |  | 8388 | 7 | Fs::DirListing { dir: hw/bsp/at32f423/FreeRTOSConfig } |  |  | 0.402 |
| walker |  | 8406 | 18 | Fs::DirListing { dir: hw/bsp/at32f423/boards/at_start_f423 } |  |  | 0.402 |
| walker |  | 8468 | 62 | Fs::DirListing { dir: hw/bsp/at32f425 } |  |  | 0.402 |
| walker |  | 8475 | 7 | Fs::DirListing { dir: hw/bsp/at32f425/FreeRTOSConfig } |  |  | 0.402 |
| walker |  | 8493 | 18 | Fs::DirListing { dir: hw/bsp/at32f425/boards/at_start_f425 } |  |  | 0.402 |
| ns | 8544 |  | 223 | src/common/tusb_types.h: inventory of named USB protocol enums | 6.1 |  | 0.397 |
| walker |  | 8556 | 63 | Fs::DirListing { dir: hw/bsp/ch32v10x } |  |  | 0.397 |
| walker |  | 8564 | 8 | Fs::DirListing { dir: hw/bsp/ch32v10x/linker } |  |  | 0.397 |
| walker |  | 8627 | 63 | Fs::DirListing { dir: hw/bsp/ch32v20x } |  |  | 0.397 |
| walker |  | 8635 | 8 | Fs::DirListing { dir: hw/bsp/ch32v20x/linker } |  |  | 0.397 |
| walker |  | 8668 | 33 | Fs::DirListing { dir: hw/bsp/ch32v20x/boards } |  |  | 0.397 |
| walker |  | 8681 | 13 | Fs::DirListing { dir: hw/bsp/ch32v20x/boards/ch32v203c_r0_1v0 } |  |  | 0.397 |
| walker |  | 8694 | 13 | Fs::DirListing { dir: hw/bsp/ch32v20x/boards/ch32v203g_r0_1v0 } |  |  | 0.397 |
| walker |  | 8707 | 13 | Fs::DirListing { dir: hw/bsp/ch32v20x/boards/nanoch32v203 } |  |  | 0.397 |
| walker |  | 8731 | 24 | Fs::DirListing { dir: hw/bsp/ch32v10x/boards/ch32v103r_r1_1v0 } |  |  | 0.397 |
| ns | 8753 |  | 209 | src/common/tusb_types.h: packed USB descriptor structs and the setup packet | 6.2 |  | 0.392 |
| walker |  | 8755 | 24 | Fs::DirListing { dir: hw/bsp/kinetis_k/boards/teensy_35 } |  |  | 0.392 |
| walker |  | 8779 | 24 | Fs::DirListing { dir: hw/bsp/stm32f0/boards/stm32f072disco } |  |  | 0.392 |
| walker |  | 8803 | 24 | Fs::DirListing { dir: hw/bsp/stm32f0/boards/stm32f072eval } |  |  | 0.392 |
| walker |  | 8827 | 24 | Fs::DirListing { dir: hw/bsp/stm32l0/boards/stm32l052dap52 } |  |  | 0.392 |
| walker |  | 8851 | 24 | Fs::DirListing { dir: hw/bsp/stm32l0/boards/stm32l0538disco } |  |  | 0.392 |
| walker |  | 8876 | 25 | Fs::DirListing { dir: hw/bsp/kinetis_k/boards/frdm_k64f } |  |  | 0.392 |
| walker |  | 8901 | 25 | Fs::DirListing { dir: hw/bsp/nuc100_120/boards/nutiny_sdk_nuc120 } |  |  | 0.392 |
| walker |  | 8926 | 25 | Fs::DirListing { dir: hw/bsp/nuc126/boards/nutiny_nuc126v } |  |  | 0.392 |
| walker |  | 8993 | 67 | Fs::DirListing { dir: hw/bsp/at32f45x } |  |  | 0.392 |
| walker |  | 9000 | 7 | Fs::DirListing { dir: hw/bsp/at32f45x/FreeRTOSConfig } |  |  | 0.392 |
| ns | 9008 |  | 255 | src/common/tusb_fifo.h: the tu_fifo_t API | 6.3 |  | 0.385 |
| walker |  | 9018 | 18 | Fs::DirListing { dir: hw/bsp/at32f45x/boards } |  |  | 0.385 |
| walker |  | 9031 | 13 | Fs::DirListing { dir: hw/bsp/at32f45x/boards/at_start_f455 } |  |  | 0.385 |
| walker |  | 9044 | 13 | Fs::DirListing { dir: hw/bsp/at32f45x/boards/at_start_f456 } |  |  | 0.385 |
| walker |  | 9057 | 13 | Fs::DirListing { dir: hw/bsp/at32f45x/boards/at_start_f457 } |  |  | 0.385 |
| walker |  | 9097 | 40 | Fs::DirListing { dir: hw/bsp/stm32h7/linker } |  |  | 0.385 |
| walker |  | 9123 | 26 | Fs::DirListing { dir: hw/bsp/lpc13/boards/lpcxpresso1347 } |  |  | 0.385 |
| walker |  | 9149 | 26 | Fs::DirListing { dir: hw/bsp/lpc15/boards/lpcxpresso1549 } |  |  | 0.385 |
| walker |  | 9175 | 26 | Fs::DirListing { dir: hw/bsp/lpc55/boards/lpcxpresso55s28 } |  |  | 0.385 |
| ns | 9196 |  | 188 | src/CMakeLists.txt: how the stack is added to a firmware build | 7.1 |  | 0.399 |
| walker |  | 9201 | 26 | Fs::DirListing { dir: hw/bsp/lpc55/boards/lpcxpresso55s69 } |  |  | 0.399 |
| walker |  | 9227 | 26 | Fs::DirListing { dir: hw/bsp/stm32n6/boards/stm32n6570dk } |  |  | 0.399 |
| walker |  | 9253 | 26 | Fs::DirListing { dir: hw/bsp/stm32n6/boards/stm32n657nucleo } |  |  | 0.399 |
| walker |  | 9325 | 72 | Fs::DirListing { dir: hw/bsp/at32f402_405 } |  |  | 0.399 |
| walker |  | 9332 | 7 | Fs::DirListing { dir: hw/bsp/at32f402_405/FreeRTOSConfig } |  |  | 0.399 |
| walker |  | 9344 | 12 | Fs::DirListing { dir: hw/bsp/at32f402_405/boards } |  |  | 0.399 |
| walker |  | 9357 | 13 | Fs::DirListing { dir: hw/bsp/at32f402_405/boards/at_start_f402 } |  |  | 0.399 |
| walker |  | 9370 | 13 | Fs::DirListing { dir: hw/bsp/at32f402_405/boards/at_start_f405 } |  |  | 0.399 |
| walker |  | 9442 | 72 | Fs::DirListing { dir: hw/bsp/at32f435_437 } |  |  | 0.399 |
| walker |  | 9449 | 7 | Fs::DirListing { dir: hw/bsp/at32f435_437/FreeRTOSConfig } |  |  | 0.399 |
| walker |  | 9461 | 12 | Fs::DirListing { dir: hw/bsp/at32f435_437/boards } |  |  | 0.399 |
| walker |  | 9474 | 13 | Fs::DirListing { dir: hw/bsp/at32f435_437/boards/at_start_f435 } |  |  | 0.399 |
| walker |  | 9487 | 13 | Fs::DirListing { dir: hw/bsp/at32f435_437/boards/at_start_f437 } |  |  | 0.399 |
| walker |  | 9514 | 27 | Fs::DirListing { dir: hw/bsp/nuc505/boards/nutiny_sdk_nuc505 } |  |  | 0.399 |
| walker |  | 9542 | 28 | Fs::DirListing { dir: hw/bsp/kinetis_k32l/boards/kuiic } |  |  | 0.399 |
| ns | 9543 |  | 347 | Example inventory: examples/ and its device, host and dual application sets | 7.2 |  | 0.394 |
| walker |  | 9570 | 28 | Fs::DirListing { dir: hw/bsp/rx/boards/rx65n_target } |  |  | 0.394 |
| walker |  | 9598 | 28 | Fs::DirListing { dir: hw/bsp/stm32g4/boards/b_g474e_dpow1 } |  |  | 0.394 |
| walker |  | 9609 | 11 | Fs::DirListing { dir: hw/bsp/stm32g4/boards/b_g474e_dpow1/cubemx } |  |  | 0.394 |
| walker |  | 9653 | 44 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards } |  |  | 0.394 |
| walker |  | 9671 | 18 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/pybadge } |  |  | 0.394 |
| ns | 9679 |  | 136 | Test inventory: unit, fuzz and hardware-in-the-loop | 7.3 |  | 0.419 |
| ns | 9686 |  | 7 | hw/ split: board-support packages vs vendor MCU SDKs | 7.4 |  | 0.421 |
| walker |  | 9689 | 18 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/pyportal } |  |  | 0.421 |
| walker |  | 9710 | 21 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/feather_m4_express } |  |  | 0.421 |
| walker |  | 9731 | 21 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/metro_m4_express } |  |  | 0.421 |
| walker |  | 9753 | 22 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/d5035_01 } |  |  | 0.421 |
| walker |  | 9775 | 22 | Fs::DirListing { dir: hw/bsp/samd5x_e5x/boards/itsybitsy_m4 } |  |  | 0.421 |
| walker |  | 9852 | 77 | Fs::DirListing { dir: hw/bsp/at32f403a_407 } |  |  | 0.421 |
| walker |  | 9859 | 7 | Fs::DirListing { dir: hw/bsp/at32f403a_407/FreeRTOSConfig } |  |  | 0.421 |
| walker |  | 9883 | 24 | Fs::DirListing { dir: hw/bsp/at32f403a_407/boards } |  |  | 0.421 |
| ns | 9884 |  | 198 | Maintenance tooling and documentation sources | 7.5 |  | 0.449 |
| walker |  | 9896 | 13 | Fs::DirListing { dir: hw/bsp/at32f403a_407/boards/at32f403a_weact_blackpill } |  |  | 0.449 |
| walker |  | 9909 | 13 | Fs::DirListing { dir: hw/bsp/at32f403a_407/boards/at_start_f403a } |  |  | 0.449 |
| walker |  | 9922 | 13 | Fs::DirListing { dir: hw/bsp/at32f403a_407/boards/at_start_f407 } |  |  | 0.449 |
| walker |  | 9951 | 29 | Fs::DirListing { dir: hw/bsp/kinetis_kl/boards/frdm_kl25z } |  |  | 0.449 |
| walker |  | 9980 | 29 | Fs::DirListing { dir: hw/bsp/rx/boards/gr_citrus } |  |  | 0.449 |
| walker |  | 9998 | 18 | Fs::DirListing { dir: hw/bsp/samg/boards/samg55_xplained } |  |  | 0.449 |
