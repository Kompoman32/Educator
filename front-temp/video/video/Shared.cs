using System;
using System.Collections.Generic;
using System.Linq;
using System.Web;
using System.Configuration;


namespace video
{
    public static class Shared
    {
        public static string GetURLNode()
        {
            return ConfigurationManager.AppSettings["URL_Node"];
        }


    }
}