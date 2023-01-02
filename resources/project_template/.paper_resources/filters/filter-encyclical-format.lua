-- First part of filter for Papal Encyclicals, done before the citations are processed.
-- Detects whether it's a second citation of the given document and suppresses the author if so.

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

local encyclical_keys = {}
local refs = {}

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
          local is_subsequent = encyclical_keys[citation.id] == true
          encyclical_keys[citation.id] = true

          if is_subsequent then
            citation.mode = "SuppressAuthor"
          end
        end
      end
      return elem
    end,
  },
}
