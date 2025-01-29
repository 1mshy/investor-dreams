import React, { useContext } from 'react';
import { SettingsContext } from '@/app/settings/SettingsContext';
import { FormControlLabel, Checkbox, Typography, Box, FormControl, InputLabel, Select, MenuItem } from '@mui/material';
import { BackGroundPaper } from '@/app/mui/theme';

export default function WidgetSettingsPanel() {
    const { settings, updateSetting } = useContext(SettingsContext);

    const renderSettingControl = (widgetKey, key, setting) => {
        // If possible_values is [true, false], render a checkbox
        if (Array.isArray(setting.possible_values) && 
            setting.possible_values.length === 2 && 
            setting.possible_values.includes(true) && 
            setting.possible_values.includes(false)) {
            return (
                <FormControlLabel
                    key={key}
                    control={
                        <Checkbox
                            checked={setting.value}
                            onChange={(e) => updateSetting(widgetKey, key, e.target.checked)}
                        />
                    }
                    label={setting.display_name}
                />
            );
        }

        // If possible_values is an array with other values, render a select
        if (Array.isArray(setting.possible_values)) {
            return (
                <Box key={key} sx={{ mb: 2 }}>
                    <FormControl>
                        <InputLabel>{setting.display_name}</InputLabel>
                        <Select
                            label={setting.display_name}
                            value={setting.value}
                            onChange={(e) => updateSetting(widgetKey, key, e.target.value)}
                            sx={{ width: 200 }}
                        >
                            {setting.possible_values.map((option) => (
                                <MenuItem key={option} value={option}>
                                    {option}
                                </MenuItem>
                            ))}
                        </Select>
                    </FormControl>
                </Box>
            );
        }

        // If possible_values is '*' or undefined, render a text display (read-only)
        return (
            <Typography key={key}>
                {setting.display_name}: {setting.value}
            </Typography>
        );
    };

    const renderWidgetSettings = (widgetKey) => {
        const widget = settings[widgetKey];
        return (
            <Box sx={{ mb: 4 }} key={widgetKey}>
                <Typography variant="h6" sx={{ mb: 2 }}>
                    {widget.display_name}
                </Typography>
                {Object.entries(widget.settings).map(([key, setting]) =>
                    renderSettingControl(widgetKey, key, setting)
                )}
            </Box>
        );
    };

    return (
        <BackGroundPaper 
            sx={{ 
                p: 3,
                height: '100vh',
                overflowY: 'auto',
                display: 'flex',
                flexDirection: 'column'
            }}
            data-tauri-drag-region
        >
            <Box 
                data-tauri-drag-region 
                sx={{ 
                    height: '32px', 
                    width: '100%', 
                    position: 'sticky',
                    top: 0,
                    zIndex: 1000
                }} 
            />
            <Typography variant="h4" sx={{ mb: 4 }} data-tauri-drag-region>Widget Settings</Typography>
            <Box sx={{ flexGrow: 1, overflowY: 'auto' }}>
                {Object.keys(settings).map((space) => renderWidgetSettings(space))}
            </Box>
        </BackGroundPaper>
    );
}
