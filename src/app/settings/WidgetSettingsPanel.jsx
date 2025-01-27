import React, { useContext } from 'react';
import { SettingsContext } from '@/app/settings/SettingsContext';
import { FormControlLabel, Checkbox, Typography, Box, Paper } from '@mui/material';
import { SoftPaper } from '@/app/mui/theme';

export default function WidgetSettingsPanel() {
    const { settings, updateSetting } = useContext(SettingsContext);

    const renderWidgetSettings = (widgetKey) => {
        const widget = settings[widgetKey];
        return (
            <Box sx={{ mb: 4 }}>
                <Typography variant="h6" sx={{ mb: 2 }}>{widget.display_name}</Typography>
                {Object.entries(widget.settings).map(([key, setting]) => (
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
                ))}
            </Box>
        );
    };

    return (
        <SoftPaper sx={{ p: 3 }}>
            <Typography variant="h4" sx={{ mb: 4 }}>Widget Settings</Typography>
            {/* Render each space */}
            {Object.keys(settings).map((space) => renderWidgetSettings(space))}
        </SoftPaper>
    );
}
