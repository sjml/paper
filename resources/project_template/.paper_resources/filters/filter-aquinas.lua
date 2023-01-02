-- On a second citation of the Summa Theologica, drop the author name so it just becomes "ST"

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

local aquinas_keys = {}
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
        if ref_data ~= nil and ref_data.author ~= nil and #ref_data.author > 0 then
          if ref_data.author[1].family == "Aquinas" and ref_data.author[1].given == "Thomas" then
            local lower_title = string.lower(pandoc.utils.stringify(ref_data.title))
            if lower_title == "summa theologica" or lower_title == "summa theologiae" then
              local is_subsequent = aquinas_keys[citation.id] == true
              aquinas_keys[citation.id] = true
              if is_subsequent then
                citation.mode = "SuppressAuthor"
              end
            end
          end
        end
      end
      return elem
    end,
  },
}
