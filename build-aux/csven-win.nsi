!include "MUI2.nsh"

Name "csven"
OutFile "csven-win-x64.exe"
Icon "C:\usr\bin\io.github.rsvzz.csven.ico"
InstallDir "$PROGRAMFILES64\csven"
!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING

Page directory
Page instfiles

Section "install"
  SetOutPath $INSTDIR
  SetOutPath "$INSTDIR\bin"
  File "C:\usr\bin\csven.exe"
  File "C:\usr\bin\*.dll"
  File "C:\usr\bin\io.github.rsvzz.csven.ico"
  SetOutPath $INSTDIR
  WriteUninstaller "Uninstall.exe"
  SetOutPath "$INSTDIR\share\icons\hicolor\scalable\apps"
  File "C:\usr\share\icons\hicolor\scalable\apps\*.svg"
  SetOutPath "$INSTDIR\share\csven\ui"
  File "C:\usr\share\csven\ui\*.ui"
  SetOutPath "$INSTDIR\share\csven\styles"
  File "C:\usr\share\csven\styles\*.css"
  SetOutPath $INSTDIR
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\bin\csven.exe"
    Delete "$INSTDIR\bin\*.dll"
    Delete "$INSTDIR\bin\*.ico"
    Delete "$INSTDIR\Uninstall.exe"
    Delete "$DESKTOP\csven.lnk"
    RMDir /r "$INSTDIR\share"
    RMDir /r "$INSTDIR\bin"
    RMDir /r "$INSTDIR"
SectionEnd

Section "Access Direct"
  CreateShortcut "$DESKTOP\csven.lnk" "$INSTDIR\bin\csven.exe" "" "$INSTDIR\bin\io.github.rsvzz.csven.ico"
SectionEnd