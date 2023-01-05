local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

if FORMAT:match("docx") then
  function Div(d)
    if utils.is_string_in_list("title-page", d.attr.classes) then
      if #d.content == 0 then
        return d
      end
      local parent = d
      local last_thing = d.content[#d.content]
      while last_thing.tag ~= "Para" do
        parent = last_thing
        last_thing = last_thing.content[#last_thing.content]
      end
      local pb = pandoc.RawBlock('openxml', '<w:p><w:r><w:br w:type="page"/></w:r></w:p>')
      table.insert(parent.content, #parent.content+1, pb)
      return d
    end
  end
end
