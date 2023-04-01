-- strips out Meta, Note, Cite, and Header elements to just give main text of Markdown,
--    for use in calculating word count of file

function Meta(m)
  return {}
end

function Cite(c)
  return {}
end

function Note(n)
  return {}
end

function Header(h)
  return {}
end
