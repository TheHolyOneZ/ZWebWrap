import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import * as api from "../api";
import { DEFAULT_LOCALE, resolveLocale, translator, type Translate } from ".";

interface Value {
  t: Translate;
  locale: string;

  setLanguage: (language: string | null) => Promise<void>;

  preference: string | null;
}

const I18nContext = createContext<Value>({
  t: translator(DEFAULT_LOCALE),
  locale: DEFAULT_LOCALE,
  setLanguage: async () => {},
  preference: null,
});

export function I18nProvider({ children }: { children: ReactNode }) {
  const [preference, setPreference] = useState<string | null>(null);

  const [locale, setLocale] = useState(() => resolveLocale(null));

  useEffect(() => {
    api
      .getPreferences()
      .then((prefs) => {
        setPreference(prefs.language ?? null);
        setLocale(resolveLocale(prefs.language));
      })
      .catch(() => {});
  }, []);

  const value = useMemo<Value>(
    () => ({
      t: translator(locale),
      locale,
      preference,
      setLanguage: async (language) => {
        setPreference(language);
        setLocale(resolveLocale(language));
        const prefs = await api.getPreferences();
        await api.setPreferences({ ...prefs, language });
      },
    }),
    [locale, preference],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): Value {
  return useContext(I18nContext);
}

export function useT(): Translate {
  return useContext(I18nContext).t;
}
