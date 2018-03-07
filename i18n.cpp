#include "i18n.h"
#include "unicode.h"
#include "convert.h"

void splitFileName(const std::wstring &fileName, std::wstring &name,
        std::wstring &ext, std::wstring &lang)
{
    int pos = fileName.find_last_of(L'.');
    if (pos <= 0) {
        ext = L"";
        name = fileName;
    } else {
        name = fileName.substr(0, pos);
        ext = fileName.substr(pos + 1);
    }

    pos = name.find_last_of('_');
    if ((pos <= 0) || (name.length() - pos != 3)) {
        lang = L"";
    } else {
        std::wstring l = name.substr(pos + 1);
        std::wstring s = name.substr(0, pos);

        name = s;
        lang = l;
    }
}

int getScore(const std::wstring &lang, const std::string &localeLang)
{
    if (! lang.length())   // locale independent
        return 1;

    if (fromMbcs(localeLang) == lang)
        return 4;

    return 0;
}
