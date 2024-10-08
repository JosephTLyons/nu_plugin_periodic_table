# nu_plugin_periodic_table

A periodic table of elements plugin for [Nushell](https://www.nushell.sh)

## Installation

1. Clone this repository
2. Build the target via `cargo build --release`
3. Add the plugin to nushell using the following nushell command `plugin add ./target/release/nu_plugin_periodic_table`
4. Reload the plugin using `plugin use periodic_table`

## Usage

```shell
〉periodic-table | where g-block == "Noble Gas" | sort-by electroneg
╭───┬───────────┬────────┬───────┬────────┬───────┬─────────────────┬───────────────────────────────────┬────────────┬─────────────┬────────────┬─────────────┬─────────┬─────────┬─────────┬───────────┬──────╮
│ # │   name    │ symbol │ a-num │ a-mass │ a-rad │     cpk-col     │            elec-config            │ electroneg │ ioniz-energ │ elec-affin │ stand-state │ m-point │ b-point │ density │  g-block  │ year │
├───┼───────────┼────────┼───────┼────────┼───────┼─────────────────┼───────────────────────────────────┼────────────┼─────────────┼────────────┼─────────────┼─────────┼─────────┼─────────┼───────────┼──────┤
│ 0 │ Helium    │ He     │     2 │   4.00 │   140 │ [217, 255, 255] │ 1s2                               │       0.00 │       24.59 │       0.00 │ Gas         │    0.95 │    4.22 │    0.00 │ Noble Gas │ 1868 │
│ 1 │ Neon      │ Ne     │    10 │  20.18 │   154 │ [179, 227, 245] │ [He]2s2 2p6                       │       0.00 │       21.57 │       0.00 │ Gas         │   24.56 │   27.07 │    0.00 │ Noble Gas │ 1898 │
│ 2 │ Argon     │ Ar     │    18 │  39.90 │   188 │ [128, 209, 227] │ [Ne]3s2 3p6                       │       0.00 │       15.76 │       0.00 │ Gas         │   83.80 │   87.30 │    0.00 │ Noble Gas │ 1894 │
│ 3 │ Radon     │ Rn     │    86 │ 222.02 │   220 │ [66, 130, 150]  │ [Xe]6s2 4f14 5d10 6p6             │       0.00 │       10.74 │       0.00 │ Gas         │  202.00 │  211.45 │    0.01 │ Noble Gas │ 1900 │
│ 4 │ Oganesson │ Og     │   118 │ 294.21 │     0 │ [0, 0, 0]       │ [Rn]7s2 7p6 5f14 6d10 (predicted) │       0.00 │        0.00 │       0.00 │ Gas         │    0.00 │    0.00 │    0.00 │ Noble Gas │ 2006 │
│ 5 │ Xenon     │ Xe     │    54 │ 131.29 │   216 │ [66, 158, 176]  │ [Kr]5s2 4d10 5p6                  │       2.60 │       12.13 │       0.00 │ Gas         │  161.36 │  165.03 │    0.01 │ Noble Gas │ 1898 │
│ 6 │ Krypton   │ Kr     │    36 │  83.80 │   202 │ [92, 184, 209]  │ [Ar]4s2 3d10 4p6                  │       3.00 │       14.00 │       0.00 │ Gas         │  115.79 │  119.93 │    0.00 │ Noble Gas │ 1898 │
╰───┴───────────┴────────┴───────┴────────┴───────┴─────────────────┴───────────────────────────────────┴────────────┴─────────────┴────────────┴─────────────┴─────────┴─────────┴─────────┴───────────┴──────╯
```

```shell
〉periodic-table -c
╭───┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────╮
│ # │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │ 9  │ 10 │ 11 │ 12 │ 13 │ 14 │ 15 │ 16 │ 17 │ 18 │
├───┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
│ 0 │ H  │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │ He │
│ 1 │ Li │ Be │    │    │    │    │    │    │    │    │    │    │ B  │ C  │ N  │ O  │ F  │ Ne │
│ 2 │ Na │ Mg │    │    │    │    │    │    │    │    │    │    │ Al │ Si │ P  │ S  │ Cl │ Ar │
│ 3 │ K  │ Ca │ Sc │ Ti │ V  │ Cr │ Mn │ Fe │ Co │ Ni │ Cu │ Zn │ Ga │ Ge │ As │ Se │ Br │ Kr │
│ 4 │ Rb │ Sr │ Y  │ Zr │ Nb │ Mo │ Tc │ Ru │ Rh │ Pd │ Ag │ Cd │ In │ Sn │ Sb │ Te │ I  │ Xe │
│ 5 │ Cs │ Ba │    │ Hf │ Ta │ W  │ Re │ Os │ Ir │ Pt │ Au │ Hg │ Tl │ Pb │ Bi │ Po │ At │ Rn │
│ 6 │ Fr │ Ra │    │ Rf │ Db │ Sg │ Bh │ Hs │ Mt │ Ds │ Rg │ Cn │ Nh │ Fl │ Mc │ Lv │ Ts │ Og │
│ 7 │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
│ 8 │    │    │ La │ Ce │ Pr │ Nd │ Pm │ Sm │ Eu │ Gd │ Tb │ Dy │ Ho │ Er │ Tm │ Yb │ Lu │    │
│ 9 │    │    │ Ac │ Th │ Pa │ U  │ Np │ Pu │ Am │ Cm │ Bk │ Cf │ Es │ Fm │ Md │ No │ Lr │    │
╰───┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────╯
```
