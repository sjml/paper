-- Second part of the Papal Encyclical filter, done after citations are processed.
--   This looks at the proper short name (e.g. "Fratelli tutti"), and looks for spans
--   in the current citation that match it without caring about case. Then it replaces
--   those spans with the proper short name to ensure that capitalization is preserved
--   and hasn't been affected by CiteProc.

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

local refs = {}

local function lower_walk(elem)
  return elem:walk({
    Str = function(s)
      return pandoc.Str(s.text:lower())
    end,
  })
end

local function fix_case(target, search_elem)
  local lowered = lower_walk(target)
  if search_elem.tag == lowered.tag then
    local lowered_search = lower_walk(search_elem)
    if lowered_search == lowered then
      return target
    end
  end
  return nil
end

local function fix_encyclical_capitalization(elem, proper)
  return elem:walk({
    Block = function(e)
      return fix_case(proper, e)
    end,
    Inline = function(e)
      return fix_case(proper, e)
    end,
    Span = function(s)
      if s.content[1].text == "“" and s.content[#s.content].text == "”" then
        s.content:remove(1)
        s.content:remove(#s.content)
        return s
      end
    end,
  })
end

return {
  {
    Pandoc = function(doc)
      refs = pandoc.utils.references(doc)
    end,
  },

  {
    Cite = function(elem)
      for _, citation in pairs(elem.citations) do
        local ref_data = utils.find_item_in_list_by_attribute(refs, "id", citation.id)
        if utils.has_keyword(ref_data, "Papal Encyclical") then
          local proper = ref_data["title-short"][1]
          return fix_encyclical_capitalization(elem, proper)
        end
      end
    end,

    Div = function(elem)
      if utils.starts_with(elem.attr.identifier, "ref-") then
        local ref_data = utils.find_item_in_list_by_attribute(refs, "id", elem.attr.identifier:sub(#"ref-" + 1))
        if not utils.has_keyword(ref_data, "Papal Encyclical") then
          return nil
        end
        local proper = ref_data["title-short"][1]
        return fix_encyclical_capitalization(elem, proper)
      end
    end,
  },
}
