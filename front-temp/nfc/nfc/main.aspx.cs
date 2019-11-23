using System;
using System.Collections.Generic;
using System.Linq;
using System.Web;
using System.Web.UI;
using System.Web.UI.WebControls;
using ProtoBuf;
using System.IO;
using System.Security.Cryptography;
using System.Net;
using System.Configuration;

namespace nfc
{
    public partial class main : System.Web.UI.Page
    {
        protected void Page_Load(object sender, EventArgs e)
        {

        }

        protected void Button1_Click(object sender, EventArgs e)
        {
            //  Проверка, еслть ли пользователь
            string sURL;
            sURL = ConfigurationManager.AppSettings["node"]+ "/api/services/educator/v1/educator/user_exist?pub_key=" + in_PKStud.Text;
            WebRequest wrGETURL;
            wrGETURL = WebRequest.Create(sURL);


            Send send = new Send();
            send.Stud = in_PKStud.Text;
            send.Lec = in_PKLec.Text;
            send.Date = DateTime.Parse(in_date.Text);

            byte[] serializeSend;
            try
            {
                using (var stream = new MemoryStream())
                {
                    Serializer.Serialize(stream, send);
                    serializeSend = stream.ToArray();
                }
            }
            catch
            {
                // Log error
                throw;
            }
            //The value to hold the signed value.
            byte[] signedHashValue;

            //Generate a public/private key pair.
            RSACryptoServiceProvider rsa = new RSACryptoServiceProvider();
            
            //Create an RSAPKCS1SignatureFormatter object and pass it the
            //RSACryptoServiceProvider to transfer the private key.
            RSAPKCS1SignatureFormatter rsaFormatter = new RSAPKCS1SignatureFormatter(rsa);

            //Set the hash algorithm to SHA1.
            rsaFormatter.SetHashAlgorithm("SHA1");



        }
    }

    [ProtoContract(SkipConstructor = true)]
    public class Send
    {
        [ProtoMember(1)]
        public string Stud { get; set; }

        [ProtoMember(2)]
        public string Lec { get; set; }

        [ProtoMember(3)]
        public DateTime Date { get; set; }
    }
}