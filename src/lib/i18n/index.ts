import { register, init, getLocaleFromNavigator, locale } from 'svelte-i18n';

register('en', () => import('./locales/en.json'));
register('es', () => import('./locales/es.json'));

export function setupI18n() {
  const initialLocale = getLocaleFromNavigator()?.startsWith('es') ? 'es' : 'en';
  
  init({
    fallbackLocale: 'en',
    initialLocale: initialLocale,
  });
}

export function switchLanguage(lang: 'es' | 'en') {
  locale.set(lang);
}
