-- Unlike the rest of Chicago style, biblical citations should be parenthetical inline.
-- The first citation needs to mention the translation being used. e.g.: (John 1:1 _NABRE_)
-- After that, the translation is dropped **unless** there are multiple translations being
--   used, in which case each citation includes the translation.
--
-- This filter will also normalize biblical book names to the preferred SBL abbreviation.
--

local utils = dofile(pandoc.path.join({ pandoc.path.directory(PANDOC_SCRIPT_FILE), "util.lua" }))

local meta = {}

-- first two in each table are "canonical" names,
--   with the third being the standard SBL abbreviation
-- stylua: ignore
local bible_books = {
  {"Genesis", "Book of Genesis", "Gen", nil, "Gn", "Ge"},
  {"Exodus", "Book of Exodus", "Ex", nil, "Ex", "Exo", "Exod"},
  {"Leviticus", "Book of Leviticus", "Lev", nil, "Lv", "Le"},
  {"Numbers", "Book of Numbers", "Num", nil, "Nb", "Nu", "Nm"},
  {"Deuteronomy", "Book of Deutoronomy", "Deut", nil, "Dt", "De"},
  {"Joshua", "Book of Joshua", "Josh", nil, "Jos", "Jsh"},
  {"Judges", "Book of Judges", "Judg", nil, "Jdg", "Jg", "Jdgs"},
  {"Ruth", "Book of Ruth", "Ruth", nil, "Rth", "Ru"},
  {"1 Samuel", "First Book of Samuel", "1 Sam", nil, "1 Sa", "1Sa", "1Sam", "1Samuel"},
  {"2 Samuel", "Second Book of Samuel", "2 Sam", nil, "2 Sa", "2Sa", "2Sam", "2Samuel"},
  {"1 Kings", "First Book of Kings", "1 Kgs", nil, "1Kgs", "1Kings"},
  {"2 Kings", "Second Book of Kings", "2 Kgs", nil, "2Kgs", "2Kings"},
  {"1 Chronicles", "First Book of Chronicles", "1 Chr", nil, "1Chr", "1 Ch", "1Ch", "1 Chron", "1Chron", "1Chronicles"},
  {"2 Chronicles", "Second Book of Chronicles", "2 Chr", nil, "2Chr", "2 Ch", "2Ch", "2 Chron", "2Chron", "2Chronicles"},
  {"Ezra", "Book of Ezra", "Ezra", nil, "Ez", "Ezr"},
  {"Nehemiah", "Book of Nehemiah", "Neh", nil, "Ne"},
  {"Esther", "Book of Esther", "Esth", nil, "Es", "Est"},
  {"Job", "Book of Job", "Job", nil, "Jb"},
  {"Psalms", "Book of Psalms", "Pss", nil, "Ps", "Psa", "Psm"},
  {"Proverbs", "Book of Proverbs", "Prov", nil, "Pr", "Pro", "Prv"},
  {"Ecclesiastes", "Book of Ecclesiastes", "Eccl", "Qoh", "Ec", "Ecc", "Eccle", "Eccles"},
  {"Song of Songs", "Song of Songs", "Song", "Cant", "So", "Canticles"},
  {"Isaiah", "Book of the Prophet Isaiah", "Isa", nil, "Is"},
  {"Jeremiah", "Book of the Prophet Jeremiah", "Jer", nil, "Je", "Jr"},
  {"Lamentations", "Book of Lamentations", "Lam", nil, "La"},
  {"Ezekial", "Book of the Prophet Ezekial", "Ezek", nil, "Eze", "Ezk"},
  {"Daniel", "Book of the Prophet Daniel", "Dan", nil, "Dn", "Da"},
  {"Hosea", "Book of the Prophet Hosea", "Hos", nil, "Ho"},
  {"Joel", "Book of the Prophet Joel", "Joel", nil, "Jl"},
  {"Amos", "Book of the Prophet Amos", "Amos", nil, "Am"},
  {"Obadiah", "Book of the Prophet Obadiah", "Obad", nil, "Ob"},
  {"Jonah", "Book of the Prophet Jonah", "Jonah", nil, "Jnh", "Jon"},
  {"Micah", "Book of the Prophet Micah", "Mic", nil, "Mc"},
  {"Nahum", "Book of the Prophet Nahum", "Nah", nil, "Na"},
  {"Habakkuk", "Book of the Prophet Habakkuk", "Hab", nil, "Hb"},
  {"Zephaniah", "Book of the Prophet Zephaniah", "Zeph", nil, "Zp", "Zep"},
  {"Haggai", "Book of the Prophet Haggai", "Hag", nil, "Hg"},
  {"Zechariah", "Book of the Prophet Zechariah", "Zech", nil, "Zc", "Zec"},
  {"Malachi", "Book of the Prophet Malachi", "Mal", nil, "Ml"},
  {"Matthew", "Holy Gospel according to Matthew", "Matt", nil, "Mt"},
  {"Mark", "Holy Gospel according to Mark", "Mark", nil, "Mk", "Mrk"},
  {"Luke", "Holy Gospel according to Luke", "Luke", nil, "Lk", "Luk"},
  {"John", "Holy Gospel according to John", "John", nil, "Jn", "Jhn"},
  {"Acts", "Acts of the Apostles", "Acts", nil, "Ac", "Act"},
  {"Romans", "Letter of Paul to the Romans", "Rom", nil, "Ro", "Rm"},
  {"1 Corinthians", "First Letter of Paul to the Corinthians", "1 Cor", nil, "1Cor", "1 Co", "1Co", "1Corinthians"},
  {"2 Corinthians", "Second Letter of Paul to the Corinthians", "2 Cor", nil, "2Cor", "2 Co", "2Co", "2Corinthians"},
  {"Galatians", "Letter of Paul to the Galatians", "Gal", nil, "Ga"},
  {"Ephesians", "Letter of Paul to the Ephesians", "Eph", nil, "Ephes"},
  {"Philippians", "Letter of Paul to the Philippians", "Phil", nil, "Php", "Pp"},
  {"Colossians", "Letter of Paul to the Colossians", "Col", nil, "Co"},
  {"1 Thessalonians", "First Letter of Paul to the Thessalonians", "1 Thess", nil, "1Thess", "1 Thes", "1Thes", "1 Th", "1Th", "1Thessalonians"},
  {"2 Thessalonians", "Second Letter of Paul to the Thessalonians", "2 Thess", nil, "2Thess", "2 Thes", "2Thes", "2 Th", "2Th", "2Thessalonians"},
  {"1 Timothy", "First Letter of Paul to Timothy", "1 Tim", nil, "1Tim", "1 Ti", "1Ti"},
  {"2 Timothy", "Second Letter of Paul to Timothy", "2 Tim", nil, "2Tim", "2 Ti", "2Ti"},
  {"Titus", "Letter of Paul to Titus", "Titus", nil, "Ti", "Tit"},
  {"Philemon", "Letter of Paul to Philemon", "Phlm", nil, "Pm", "Philem", "Phm"},
  {"Hebrews", "Letter to the Hebrews", "Heb"},
  {"James", "Letter of James", "Jas", nil, "Jm"},
  {"1 Peter", "First Letter of Peter", "1 Pet", nil, "1Pet", "1 Pe", "1Pe", "1 Pt", "1Pt", "1Peter"},
  {"2 Peter", "Second Letter of Peter", "2 Pet", nil, "2Pet", "2 Pe", "2Pe", "2 Pt", "2Pt", "2Peter"},
  {"1 John", "First Letter of John", "1 John", nil, "1John", "1 Jn", "1Jn", "1 Jhn", "1Jhn"},
  {"2 John", "Second Letter of John", "2 John", nil, "2John", "2 Jn", "2Jn", "2 Jhn", "2Jhn"},
  {"3 John", "Third Letter of John", "3 John", nil, "3John", "3 Jn", "3Jn", "3 Jhn", "3Jhn"},
  {"Jude", "Letter of Jude", "Jude", nil, "Jd", "Jud"},
  {"Revelation", "Book of Revelation", "Rev", nil, "Re", "Rv"},
  {"Baruch", "Book of Baruch", "Bar"},
  {"1 Esdras", "First Book of Esdras", "1 Esd", nil, "1Esd", "1 Esdr", "1Esdr", "1Esdras"},
  {"2 Esdras", "Second Book of Esdras", "2 Esd", nil, "2Esd", "2 Esdr", "2Esdr", "2Esdras"},
  {"Judith", "Book of Judith", "Jdt", nil, "Jth", "Jdth"},
  {"1 Maccabees", "First Book of Maccabees", "1 Macc", nil, "1Macc", "1 Mac", "1Mac", "1 Ma", "1Ma", "1Maccabees"},
  {"2 Maccabees", "Second Book of Maccabees", "2 Macc", nil, "2Macc", "2 Mac", "2Mac", "2 Ma", "2Ma", "2Maccabees"},
  {"3 Maccabees", "Third Book of Maccabees", "3 Macc", nil, "3Macc", "3 Mac", "3Mac", "3 Ma", "3Ma", "3Maccabees"},
  {"4 Maccabees", "Fourth Book of Maccabees", "4 Macc", nil, "4Macc", "4 Mac", "4Mac", "4 Ma", "4Ma", "4Maccabees"},
  {"Sirach", "Book of Sirach", "Sir"},
  {"Tobit", "Book of Tobit", "Tob", nil, "Tb"},
  {"Wisdom", "Book of Wisdom", "Wis", nil, "Ws"}
}

function normalize_book_name(book, idx)
  if idx > 3 then
    print("[LUA FILTER WARNING] Cannot normalize a Bible book to an index greater than 3.")
    return nil
  end

  for _, list in pairs(bible_books) do
    for _, bk in pairs(list) do
      if bk == book then
        local suff = ""
        if list[1] ~= list[3] then
          suff = "."
        end
        return list[idx] .. suff
      end
    end
  end

  print('[LUA FILTER WARNING] Could not normalize book name: "' .. book .. '"')
  return nil
end

function process_bible_citation(suffix)
  local suffix = utils.strip(suffix)
  if suffix:sub(1, 1) == "," then
    suffix = suffix:sub(2)
    suffix = utils.strip(suffix)
  end
  local book = ""
  local range = ""

  for tok in string.gmatch(suffix, "[^%s]+") do
    if #book == 0 then
      book = tok
    elseif #range > 0 then
      range = range .. " " .. tok
    else
      if tonumber(tok:sub(1, 1)) ~= nil then
        range = tok
      else
        book = book .. " " .. tok
      end
    end
  end
  book = utils.strip(book)
  if #book == 0 then
    print("[LUA FILTER WARNING] No reference given for bible citation.")
    return nil
  end
  book = normalize_book_name(book, 3)
  if book == nil then
    return nil
  end
  range = utils.strip(range)

  return book .. " " .. range
end

local bible_translations = {}

function filter_cite_element_for_biblical_refs(elem)
  local translations = {}
  local bible_cites = {}
  local other_count = 0
  local note_num = nil
  for _, citation in pairs(elem.citations) do
    if utils.starts_with(citation.id, "Bible-") then
      local translation = citation.id:sub(#"Bible-" + 1)
      translations[translation] = 1
      local bcite = process_bible_citation(pandoc.utils.stringify(citation.suffix))
      table.insert(bible_cites, bcite)
      note_num = citation.note_num
    else
      other_count = other_count + 1
    end
  end
  local translation_list = {}
  for t, _ in pairs(translations) do
    table.insert(translation_list, t)
  end
  if #translation_list > 1 then
    print(
      "[LUA FILTER WARNING] Cannot mix translations in a single citation. ("
        .. table.concat(translation_list, " + ")
        .. ")"
    )
    return nil
  end

  if #bible_cites > 0 and other_count > 0 then
    print("[LUA FILTER WARNING] Cannot mix Bible citations with other types.")
    return nil
  end

  if other_count > 0 then
    return nil
  end

  local translation = translation_list[1]
  local pseudo_cite = {}
  table.insert(pseudo_cite, pandoc.Str("("))
  for i, c in pairs(bible_cites) do
    table.insert(pseudo_cite, pandoc.Str(c))
    if i < #bible_cites then
      table.insert(pseudo_cite, pandoc.Str(";"))
      table.insert(pseudo_cite, pandoc.Space())
    end
  end
  table.insert(
    pseudo_cite,
    pandoc.Span({ pandoc.Space(), pandoc.Emph(pandoc.Str(translation)) }, { class = "bible-translation-cite" })
  )
  table.insert(pseudo_cite, pandoc.Str(")"))

  if translation == "Vulgatam" and bible_translations[translation] ~= true then
    if meta.vulgate_cite_key == nil then
      print("[LUA FILTER WARNING] Vulgatam citation made, but no vulgate_cite_key given in metadata.")
      return nil
    end
    local vcitation = pandoc.Citation(meta.vulgate_cite_key, "NormalCitation")
    vcitation.note_num = note_num
    local vcite = pandoc.Cite({}, { vcitation })
    table.insert(pseudo_cite, vcite)
  end

  bible_translations[translation] = true

  local citespan = pandoc.Span(pseudo_cite, { class = "bible-cite" })
  return citespan
end

function filter_span_elements_for_translation_reference(elem)
  if utils.is_string_in_list("bible-translation-cite", elem.classes) then
    -- if using more than one translation, name it every time
    if utils.count_table_entries(bible_translations) > 1 then
      return nil
    end
    local tkey = utils.strip(pandoc.utils.stringify(elem))
    -- if it's the Vulgate, only name on subsequents
    if tkey == "Vulgatam" then
      if bible_translations[tkey] == true then
        bible_translations[tkey] = false
        return {}
      else
        return nil
      end
    end
    -- otherwise, only name it the first time
    if bible_translations[tkey] == true then
      bible_translations[tkey] = false
      return nil
    else
      return {}
    end
  end
end

function add_spaces_before_cites(elem)
  for i = 1, #elem - 1 do
    if elem[i + 1].tag == "Span" and utils.is_string_in_list("bible-cite", elem[i + 1].classes) then
      if elem[i].tag ~= "Space" then
        table.insert(elem, i + 1, pandoc.Space())
        return elem
      end
    end
  end
end

-- Four passes:
--   - grab the metadata
--   - make the citations
--   - clean up translation mentions
--   - add a space if needed
return {
  {
    Meta = function(m)
      utils.fix_table_strings(m)
      meta = m
    end,
  },
  { Cite = filter_cite_element_for_biblical_refs },
  { Span = filter_span_elements_for_translation_reference },
  { Inlines = add_spaces_before_cites },
}
