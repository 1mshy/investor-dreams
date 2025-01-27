import React, { createContext, useState, useEffect } from 'react';
import { user_settings as defaultSettings } from '@/app/settings/settings';
import localforage from 'localforage';

// Create a dedicated localforage instance for settings
const SETTINGS_CACHE = localforage.createInstance({
    name: "user_settings"
});

export const SettingsContext = createContext();

export const SettingsProvider = ({ children }) => {
    const [settings, setSettings] = useState(defaultSettings);

    useEffect(() => {
        // Load settings from localforage on mount
        const loadSettings = async () => {
            try {
                const stored = await SETTINGS_CACHE.getItem('appSettings');
                if (stored) {
                    setSettings(stored);
                }
            } catch (e) {
                console.error('Error loading settings from localforage:', e);
            }
        };
        loadSettings();
    }, []);

    useEffect(() => {
        // Save settings to localforage whenever they change
        const saveSettings = async () => {
            try {
                await SETTINGS_CACHE.setItem('appSettings', settings);
            } catch (e) {
                console.error('Error saving settings to localforage:', e);
            }
        };
        saveSettings();
    }, [settings]);

    const updateSetting = (widget, settingKey, newValue) => {
        setSettings(prev => {
            const newSettings = { ...prev };
            newSettings[widget].settings[settingKey].value = newValue;
            return newSettings;
        });
    };

    return (
        <SettingsContext.Provider value={{ settings, updateSetting }}>
            {children}
        </SettingsContext.Provider>
    );
};
