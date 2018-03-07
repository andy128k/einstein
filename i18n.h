#ifndef __I18N_H__
#define __I18N_H__


/// \file i18n.h
/// Locale related functions


#include <string>

extern "C" {
    const char * ein_get_language();
}


// split file name to file name, extension, language name and country
// for exmaple, "story_ru_RU.txt" shoud be splited to
// name="story", extension="txt", language="ru", country="RU"
void splitFileName(const std::wstring &fileName, std::wstring &name,
        std::wstring &ext, std::wstring &lang);

// calculate relevance score between language, country and
// current locale
int getScore(const std::wstring &lang, const std::string &localeLang);

#endif
