import Box from '@mui/material/Box';
import Slider from '@mui/material/Slider';

type Mark = {
    value: number;
    label: string;
}

export default function DiscreteSlider({
    ariaLabel,
    value,
    step,
    min,
    max,
    marks,
    unit,
    onChange,
} : {
    ariaLabel: string,
    value: number;
    step: number;
    min: number;
    max: number;
    marks?: Mark[];
    unit?: string;
    onChange?: (value: number) => void;
}) {
    return (
        <Box sx={{ width: "100%" }}>
            <Slider
                aria-label={ariaLabel}
                value={value}
                valueLabelDisplay="auto"
                valueLabelFormat={(value) => `${value} ${unit}`}
                shiftStep={30}
                step={step}
                min={min}
                max={max}
                marks={marks}
                onChange={(_, value) => {
                    if (typeof value === "number") {
                        onChange?.(value);
                    }
                }}
                sx={{
                    // track (filled part)
                    "& .MuiSlider-track": {
                        backgroundColor: "#3b82f6", // blue
                        height: 6,
                    },

                    // rail (background)
                    "& .MuiSlider-rail": {
                        opacity: 0.4,
                        height: 6,
                    },

                    // thumb (circle)
                    "& .MuiSlider-thumb": {
                        width: 22,
                        height: 22,
                        backgroundColor: "#3b82f6",
                    },

                    // floating value label
                    "& .MuiSlider-valueLabel": {
                        backgroundColor: "#1e293b",
                        color: "#fff",
                        fontFamily: "monospace",
                        fontSize: "0.85rem",
                        borderRadius: "8px",
                    },

                    // mark labels (1, 2, 4, 8, 12, 16)
                    "& .MuiSlider-markLabel": {
                        color: "#e5e7eb",
                        fontFamily: "monospace",
                        fontSize: "0.75rem",
                    },

                    // active mark label
                    "& .MuiSlider-markLabelActive": {
                        color: "#fbbf24", // amber
                        fontWeight: 600,
                    },
                }}
            />
            {/* <Slider defaultValue={30} step={10} marks min={10} max={110} disabled /> */}
        </Box>
    );
}
