-- miscellaneous utility functions that get used by other filters.

-- recursive functions can't be anonymous; have to live outside the returned table
local function dump(t, prefix)
  if t == nil then
    print("nil")
    return
  end
  if prefix == nil then
    prefix = ""
  end
  if type(t) ~= "table" then
    print(prefix .. t)
  end
  for k, v in pairs(t) do
    if type(v) == "table" then
      print(prefix .. k)
      dump(v, prefix .. "    ")
    end
    print(prefix .. k, v)
  end
end

local function fix_table_strings(t)
  for k, v in pairs(t) do
    if type(v) == "table" then
      if pandoc.utils.type(v) == "Inlines" then
        t[k] = pandoc.utils.stringify(v)
      else
        fix_table_strings(t[k])
      end
    end
  end
end

-- so they can be "dofile"-ed into a variable and not pollute the global namespace
return {
  dump = dump,
  fix_table_strings = fix_table_strings,

  starts_with = function(str, start)
    return str:sub(1, #start) == start
  end,

  ends_with = function(str, ending)
    return ending == "" or str:sub(-#ending) == ending
  end,

  strip = function(str)
    return (str:gsub("^%s*(.-)%s*$", "%1"))
  end,

  is_str_truthy = function(str)
    if str == nil then
      return false
    end
    numval = tonumber(str)
    if numval ~= nil then
      return numval ~= 0
    end
    str = string.lower(str)
    if str == "true" or str == "t" then
      return true
    elseif str == "yes" or str == "y" then
      return true
    end
    return false
  end,

  is_string_in_list = function(str, list)
    for _, s in pairs(list) do
      if s == str then
        return true
      end
    end
    return false
  end,

  count_table_entries = function(tbl)
    count = 0
    for _ in pairs(tbl) do
      count = count + 1
    end
    return count
  end,

  find_item_in_list_by_attribute = function(list, attr, value)
    if #list == 0 then
      return nil
    end
    if attr == nil or value == nil then
      return nil
    end

    for _, item in pairs(list) do
      if item[attr] == value then
        return item
      end
    end
    return nil
  end,

  has_keyword = function(ref_data, keyword)
    if ref_data == nil then
      return false
    end
    if ref_data.keyword == nil then
      return false
    end
    entry_kw = pandoc.utils.stringify(ref_data.keyword)
    if entry_kw:find(keyword, 1, true) ~= nil then
      return true
    end
    return false
  end,
}
