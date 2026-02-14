; ATX headings (# Heading, ## Heading, etc.)
(atx_heading
  (inline) @name) @symbol

; Setext headings (underlined with === or ---)
(setext_heading
  (paragraph) @name) @symbol
