workspace("FRAME")
    configurations { "Debug", "Release" }

project "frame_compiler"
    kind "ConsoleApp"
    language "C++"
    cppdialect "c++Latest"

    outputdir = "%{cfg.buildcfg}-%{cfg.system}-%{cfg.architecture}"

    targetdir ("%{wks.location}/bin/" .. outputdir .. "/%{prj.name}")
    objdir ("%{wks.location}/bin-int/" .. outputdir .. "/%{prj.name}")

    includedirs { "$(llalsd)" }

--    links { "" }

    files { "src/**.h", "src/**.cpp" }
