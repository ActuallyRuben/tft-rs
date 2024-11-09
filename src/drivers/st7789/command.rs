#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Command {
    /// No Operation
    NOP = 0x00,
    /// Software Reset
    SWRESET = 0x01,
    /// Read Display ID
    RDDID = 0x04,
    /// Read Display Status
    RDDST = 0x09,
    /// Read Display Power Mode
    RDDPM = 0x0A,
    /// Read Display MADCTL
    RDDMADCTL = 0x0B,
    /// Read Display Pixel Format
    RDDCOLMOD = 0x0C,
    /// Read Display Image Mode
    RDDIM = 0x0D,
    /// Read Display Signal Mode
    RDDSDR = 0x0F,
    /// Read Display Self-Diagnostic Resul
    SLPIN = 0x10,
    /// Sleep in
    SLPOUT = 0x11,
    /// Sleep Out
    PTLON = 0x12,
    /// Partial Display Mode On
    NORON = 0x13,
    /// Display Inversion Off
    INVOFF = 0x20,
    /// Display Inversion O
    INVON = 0x21,
    /// Gamma Set
    GAMSET = 0x26,
    /// Display Off
    DISPOFF = 0x28,
    /// Display On
    DISPON = 0x29,
    /// Column Address Set
    CASET = 0x2A,
    /// Row Address Set
    RASET = 0x2B,
    /// Memory Write
    RAMWR = 0x2C,
    /// Memory Read
    RAMRD = 0x2E,
    /// Partial Area
    PTLAR = 0x30,
    /// Vertical Scrolling Definition
    VSCRDEF = 0x33,
    /// Tearing Effect Line OFF
    TEOFF = 0x34,
    /// Tearing Effect Line On
    TEON = 0x35,
    /// Memory Data Access Control
    MADCTL = 0x36,
    /// Vertical Scroll Start Address of RAM
    VSCSAD = 0x37,
    /// Idle Mode Off
    IDMOFF = 0x38,
    /// Idle mode on
    IDMON = 0x39,
    /// Interface Pixel Format
    COLMOD = 0x3A,
    /// Write Memory Continue
    WRMEMC = 0x3C,
    /// Read Memory Continue
    RDMEMC = 0x3E,
    /// Set Tear Scanline
    STE = 0x44,
    /// Get Scanlin
    GSCAN = 0x45,
    /// Write Display Brightness
    WRDISBV = 0x51,
    /// Read Display Brightness Value
    RDDISBV = 0x52,
    /// Write CTRL Display
    WRCTRLD = 0x53,
    /// Read CTRL Value Display
    RDCTRLD = 0x54,
    /// Write Content Adaptive Brightness Control and Color Enhancement
    WRCACE = 0x55,
    /// Read Content Adaptive Brightness Control
    RDCABC = 0x56,
    /// Write CABC Minimum Brightness
    WRCABCMB = 0x5E,
    /// Read CABC Minimum Brightness
    RDCABCMB = 0x5F,
    /// Read Automatic Brightness Control Self-Diagnostic Result
    RDABCSDR = 0x68,
    /// Read ID1
    RDID1 = 0xDA,
    /// Read ID2
    RDID2 = 0xDB,
    /// Read ID3
    RDID3 = 0xDC,
    /// RAM Control
    RAMCTRL = 0xB0,
    /// RGB Interface Control
    RGBCTRL = 0xB1,
    /// Porch Setting
    PORCTRL = 0xB2,
    /// Frame Rate Control 1 (In partial mode/ idle colors)
    FRCTRL1 = 0xB3,
    /// Partial Control
    PARCTRL = 0xB5,
    /// Undocumented command
    UNKNOWN = 0xB6,
    /// Gate Control
    GCTRL = 0xB7,
    /// Gate On Timing Adjustment
    GTADJ = 0xB8,
    /// Digital Gamma Enable
    DGMEN = 0xBA,
    /// VCOM Setting
    VCOMS = 0xBB,
    /// Power Saving Mode
    POWSAVE = 0xBC,
    /// Display off power save
    DLPOFFSAVE = 0xBD,
    /// LCM Control
    LCMCTRL = 0xC0,
    /// ID Code Setting
    IDSET = 0xC1,
    /// VDV and VRH Command Enable
    VDVVRHEN = 0xC2,
    /// VRH Set
    VRHS = 0xC3,
    /// VDV Set
    VDVS = 0xC4,
    /// VCOM Offset Set
    VCMOFSET = 0xC5,
    /// Frame Rate Control in Normal Mode
    FRCTRL2 = 0xC6,
    /// CABC Control
    CABCCTRL = 0xC7,
    /// Register Value Selection 1
    REGSEL1 = 0xC8,
    /// Register Value Selection 2
    REGSEL2 = 0xCA,
    /// PWM Frequency Selection
    PWMFRSEL = 0xCC,
    /// Power Control 1
    PWCTRL1 = 0xD0,
    /// Enable VAP/VAN signal output
    VAPVANEN = 0xD2,
    /// Command 2 Enable
    CMD2EN = 0xDF,
    /// Positive Voltage Gamma Control
    PVGAMCTRL = 0xE0,
    /// Negative Voltage Gamma Control
    NVGAMCTRL = 0xE1,
    /// Digital Gamma Look-up Table for Red
    DGMLUTR = 0xE2,
    /// Digital Gamma Look-up Table for Blue
    DGMLUTB = 0xE3,
    /// Gate Control
    GATECTRL = 0xE4,
    /// SPI2 Enable
    SPI2EN = 0xE7,
    /// Power Control 2
    PWCTRL2 = 0xE8,
    /// Equalize time control
    EQCTRL = 0xE9,
    /// Program Mode Control
    PROMCTRL = 0xEC,
    /// Program Mode Enable
    PROMEN = 0xFA,
    /// NVM Setting
    NVMSET = 0xFC,
    /// Program action
    PROMACT = 0xFE,
}
