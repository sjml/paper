-- First part of filter for abbreviating institutional names, done before the citations are processed.
--   In this pass, it just suppresses the author entirely. The abbreviated version will
--   be added back in the post-citation pass.

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))
dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "institutional-abbreviations.lua" }))

local refs = {}
local seen_authors = {}

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
        if
          ref_data ~= nil
          and ref_data.author ~= nil
          and #ref_data.author > 0
          and ref_data.author[1].literal ~= nil
        then
          local auth = pandoc.utils.stringify(ref_data.author)
          if seen_authors[auth] == true and institutional_abbreviations[auth] ~= nil then
            citation.mode = "SuppressAuthor"
          end

          seen_authors[auth] = true
        end
      end
      return elem
    end,
  },
}
