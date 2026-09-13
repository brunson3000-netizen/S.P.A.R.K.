# ast-grep / Tree-sitter Failure Lessons

Sources:
- ast-grep @ `45b5eb6705b4c24e04746137d259874abf1087ad`
- Tree-sitter @ `1b8407d1e718f2a26e2886c03cc55622d8d1d7bd`

## SLICE-F01 — Recovered syntax tree can look more certain than the source deserves

Tree-sitter intentionally recovers from malformed/incomplete code using `ERROR` and missing nodes. A structural match can therefore exist in a tree that is not cleanly parsed.

SPARK lesson: attach parse-health/error-overlap state to structural evidence. A recovered-tree match is evidence, not semantic proof.

## SLICE-F02 — Separate upstream pins are not automatically one dependency set

Harvested Tree-sitter is 0.28.0; ast-grep 0.45.3 declares Tree-sitter 0.27.0.

SPARK lesson: keep exact dependency compatibility. Do not “upgrade” an embedded library merely because a newer independently harvested upstream exists.

## SLICE-F03 — Structural search is not semantic resolution

AST patterns can reliably identify syntax shapes/ranges but do not by themselves prove type identity, dynamic dispatch, alias resolution or whole-program data flow.

SPARK lesson: label evidence by capability class. `syntax_match` must not be surfaced as `semantic_reference` without a semantic layer.

## SLICE-F04 — Read-only search and rewrite are different authorities

ast-grep can produce replacements/diffs, but context-slicing research needs only search.

SPARK lesson: expose read-only structural inspection first. Mutation requires a separate capability/grant and independent patch validation.
