# Fasta tools

## find-pep

```bash
A CLI tool for find peptides in UniProt fasta files

Usage: find-pep <INPUT> <LIST>

Arguments:
  <INPUT>  Path to the input FASTA file (required)
  <LIST>   Path to the Input peptide list file (required)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- Example

  ```bash
  find-pep input_fasta.fasta input_list.txt | tee output.txt
  DTLMISR:F7HL06:1
  DTLMISR:A3RFZ7-3:1
  DTLMISR:F7F0D6:1
  ISRNQVSLTCLVK:F7HL06:1
  ISRNQVSLTCLVK:B0FPE9:2
  ISRNQVSLTCLVK:F7F0D6:1
  ```

## fasta-extractor

```bash
Extract FASTA headers from a file.

Usage: fasta-extractor [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the input FASTA file (required)

Options:
  -n, --njobs <NJOBS>  Number of parallel jobs (default: all available CPUs)
  -h, --help           Print help
  -V, --version        Print version
```

- Example

  ```bash
  fasta-extractor input_fasta.fasta | tee output.txt
  Extracting headers from FASTA file... using 10 jobs
  >tr|F7HL06|F7HL06_MACMU Ig-like domain-containing protein OS=Macaca mulatta OX=9544 PE=4 SV=3
  >tr|F7F0D6|F7F0D6_MACMU Ig-like domain-containing protein OS=Macaca mulatta OX=9544 PE=4 SV=3
  >sp|Q63ZW7-3|INADL_MOUSE Isoform 3 of InaD-like protein OS=Mus musculus OX=10090 GN=Patj
  >sp|B0FPE9|NLRP3_MACMU NACHT, LRR and PYD domains-containing protein 3 OS=Macaca mulatta OX=9544 GN=NLRP3 PE=2 SV=1
  >sp|A3RFZ7-3|FCG3A_MACMU Isoform 3 of Low affinity immunoglobulin gamma Fc region receptor III-A OS=Macaca mulatta OX=9544 GN=FCGR3A
  ```

## Bumping Version

Bump the version number by running `cargo v [part]` where `[part]` is `major`, `minor`, or `patch`, depending on which part of the version number you want to bump.

```bash
cargo install cargo-v

# commit
cargo v patch -y
# push
cargo build --release -j 10
git push origin --tags
```

## Changelog

- 0.2.0
  - Added `fasta-extractor`
- 0.1.0
  - Initial release with `find-pep`
