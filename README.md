# gcc-emu, a Linux compilation emulator

sample output, printed with a random delay of 0–3 seconds between invocations

    gcc -mrenesas -fcaller-saves -mno-power8-vector -nodevicelib -merror-reloc -mv850e2v3 -Wshift-negative-value -mlong64 -mmfcrf
    gcc -mips32r5 -mno-eva -mno-abicalls -undefined -mavx512pf -fstack-protector-all -1 -CC -m16-bit
    gcc -mmvme -msynci -mno-fp-ret-in-387 -m32r -fipa-vrp -Wold-style-definition -Wredundant-decls
    gcc -mno-default -fno-peephole2 -F -mtomcat-stats -fstats -mbionic -fauto-profile -fno-dwarf2-cfi-asm -mno-toc -m3dnow -mfix-rm7000 -mno-mt
    gcc -undefined -mno-cmov -mmfcrf -v -m2e -mips64r3
    gcc -mno-prototype -gxcoff -mxgot -mno-optimize-membar -mips64r5 -O -mno-micromips -lz -fPIE -mno-dsp -mARC600 -mno-synci -pedantic-errors
    gcc -mcorea -EB -mv850e3v5 --args -O1 -sim --no-sysroot-suffix -EB -msep-data -mpopcntb -mpopcntb -Wmain -mcx16 -fschedule-insns2 -mno-thumb-interwork -mstrict-align -mspe -Wvla
    gcc -m64 -mlong-double-128 -Wformat -P -fmem-report-wpa -mas100-syntax -mmainkernel -mfloat-ieee -melinux -mgp64 -Wnon-virtual-dtor -fopt-info-vec-missed -mfp32 -m16bit -fipa-sra -mlra
    gcc -m3 -munaligned-doubles -mno-compat-align-parm -mno-float128 -mauto-litpools -mcbranch-force-delay-slot -mcorea -mbig-endian -mno-div -symbolic -ftree-copy-prop -mdspr2 -mhard-dfp -mel -Wmain -mfma
    gcc -mfloat32 -- -msse3 -Wpointer-sign -ffinite-math-only -mno-odd-spreg -mno-fsca -m2e -fopt-info-vec-missed -m96bit-long-double -m4-single -mno-vis4
    gcc -mv850es -mno-callgraph-data -mno-dpfp-lrsr -mno-mdmx -mxgot -mno-long-calls -mio-volatile -mlarge-mem -mauto-litpools -fpost-ipa-mem-report -mlong-calls -mno-msa -Wmemset-transposed-args -mno-cond-exec -mpoke-function-name -O1 -Wredundant-decls -mads

prereqs:

* `man`
* a man page for `gcc` (test: `man gcc`), usually easiest to get by installing
  gcc (`sudo apt-get install build-essential` in some systems, `gcc-core` on
  cygwin, etc. etc.)

coming soon:

* a `./configure` stage
* filenames, generated... somehow? im fine using a corpus or combining name
  fragments; there’s some in the `data` directory already
* compilation warnings
* a link stage
* an install stage
