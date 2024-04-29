@echo off
rem copy book.js to theme/
if not exist "theme" mkdir theme
copy mdbook_plugin\book.js theme\book.js >nul

if not exist "src" mkdir src

rem clear assets and md files to mdbook directory
del src\*md
if exist "src\assets" rmdir /s /q src\assets

rem Write the first line of SUMMARY.md. This clears anything that was there previously
echo # Summary > src\SUMMARY.md

echo Generating visualizations for the following examples:

rem Uncomment the examples are being tested
set targetExamples=^
    copy ^
        func_take_ownership ^
        func_take_return_ownership ^
        function ^
        hatra1 ^
        hatra1_test ^
        hatra2 ^
        immutable_borrow ^
        immutable_borrow_lifetime ^
        immutable_borrow_method_call ^
        immutable_variable ^
        move_assignment ^
        move_different_scope ^
        move_func_return ^
        multiple_immutable_borrow ^
        mutable_borrow ^
        mutable_borrow_method_call ^
        mutable_variables ^
        nll_lexical_scope_different ^
        printing ^
        string_from_move_print ^
        string_from_print ^
        struct_lifetime ^
        struct_rect ^
        struct_rect2 ^
        struct_string ^
        extra_credit ^
        beispiel1 ^
        test

set EX=..\src\examples
rem Loop through the specified examples
rem Loop through the specified examples
for %%t in (%targetExamples%) do (
    echo|set /p="building %%t..."

    rem Check if required files are there
    if exist "%EX%\%%t\input\annotated_source.rs" if exist "%EX%\%%t\source.rs" (
        rem Check if file headers exist
        if not exist "%EX%\%%t\main.rs" (
            echo generating header for %%t...
            cd ..\RustvizParse
            cargo run "%EX%\%%t\source.rs" >nul 2>&1
        )


        cd ..\src
        cargo run "%%t" >nul 2>&1

        set failed=0
        if %errorlevel% NEQ 0 ( failed = 1 )
        if not exist "examples\%%t\vis_code.svg" ( failed = 1 )
        if not exist "examples\%%t\vis_timeline.svg" ( failed = 1 )

        if failed EQU 1 (
            echo FAILED on SVG generation.
            cd ..\rustviz_mdbook
        ) else (
            cd ..\rustviz_mdbook
            rem Copy files to mdbook directory
            mkdir "%~dp0\src\assets\%%t" >nul 2>&1
            copy /Y "%EX%\%%t\source.rs" "%~dp0\src\assets\%%t\source.rs" >nul 2>&1
            copy /Y "%EX%\%%t\vis_code.svg" "%~dp0\src\assets\%%t\vis_code.svg" >nul 2>&1
            copy /Y "%EX%\%%t\vis_timeline.svg" "%~dp0\src\assets\%%t\vis_timeline.svg" >nul 2>&1

            rem Add append corresponding line to SUMMARY.md
            echo|set /p="- [%%t](./%%t.md)" >> src\SUMMARY.md
            echo. >> src\SUMMARY.md
            echo. done

            echo ### %%t >> src\%%t.md
            echo ```rust >> src\%%t.md
            echo {{#rustdoc_include assets\%%t\source.rs}} >> src\%%t.md
            echo ``` >> src\%%t.md

            echo ^<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display: flex;"^> >> src\%%t.md
            echo     ^<object type="image/svg+xml" class="%%t code_panel" data="assets\%%t\vis_code.svg"^>^</object^> >> src\%%t.md
            echo     ^<object type="image/svg+xml" class="%%t tl_panel" data="assets\%%t\vis_timeline.svg" style="width: auto;" onmouseenter="helpers(%%t)"^>^</object^> >> src\%%t.md
            echo ^</div^> >> src\%%t.md
        )
    )
)

rem Build mdbook
mdbook build

rem Run HTTP server on docs directory
mdbook serve -p 8000
