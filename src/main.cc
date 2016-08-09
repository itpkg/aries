int main(int argc, char** argv){
  return 0;
}
// #include "../src/app.h"
//
// #include <iostream>
// #include <gflags/gflags.h>
//
// DEFINE_bool(big_menu, true, "Include 'advanced' options in the menu listing");
// DEFINE_string(languages, "english,french,german",
//                  "comma-separated list of languages to offer in the 'lang' menu");
//
// int main(int argc, char** argv){
//     gflags::SetVersionString("2016.08.01");
//     gflags::SetUsageMessage("Usage : ./demo ");
//     google::ParseCommandLineFlags(&argc, &argv, true);
//
//     std::cout << "argc=" << argc << std::endl;
//     if (FLAGS_big_menu) {
//       std::cout << "big menu is ture" << std::endl;
//     } else {
//       std::cout << "big menu is flase" << std::endl;
//     }
//
//     std::cout << "languages=" << FLAGS_languages << std::endl;
//
//      gflags::ShutDownCommandLineFlags();
//
//   // aries::App app;
//   // app.start(argc, argv);
//   return 0;
// }
