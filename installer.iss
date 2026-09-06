; TapDeck — Installer Inno Setup
; Compila con:  ISCC.exe installer.iss
; Prerequisito: target\release\tapdeck.exe compilato (cargo build --release)

#ifndef MyAppVersion
  #define MyAppVersion "0.1.0"
#endif
#ifndef MyAppName
  #define MyAppName "TapDeck"
#endif
#ifndef MyAppExe
  #define MyAppExe "tapdeck.exe"
#endif
#ifndef MyAppId
  #define MyAppId "{{8E7A0C4B-6D2F-4A9B-9E5C-1F3B8D2A7E64}"
#endif

[Setup]
AppId={#MyAppId}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher=TapDeck
AppPublisherURL=https://github.com/xqm67/tapdeck
AppSupportURL=https://github.com/xqm67/tapdeck
AppUpdatesURL=https://github.com/xqm67/tapdeck
AppComments=TapDeck — la tua Stream Deck fatta in casa
AppContact=tapdeck@example.com
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
AllowNoIcons=yes
LicenseFile=LICENSE
OutputDir=dist
OutputBaseFilename=TapDeck-Setup-{#MyAppVersion}
SetupIconFile=res\icon.ico
UninstallDisplayIcon={app}\{#MyAppExe}
UninstallDisplayName={#MyAppName}
Compression=lzma2/ultra
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x64compatible
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
UsePreviousAppDir=yes
MinVersion=6.1sp1

[Languages]
Name: "en"; MessagesFile: "compiler:Default.isl"
Name: "it"; MessagesFile: "compiler:Languages\Italian.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startmenu";  Description: "{cm:CreateStartMenuIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[CustomMessages]
en.LangCode=en
it.LangCode=it
en.CreateStartMenuIcon=Create a &Start Menu shortcut
it.CreateStartMenuIcon=Crea una scorciatoia nel &menu Start

[Files]
Source: "target\release\{#MyAppExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "res\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExe}"; Tasks: startmenu
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExe}"; Tasks: desktopicon; IconFilename: "{app}\icon.ico"

[Run]
Filename: "{app}\{#MyAppExe}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
