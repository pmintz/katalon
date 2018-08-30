<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_var fndBtn Yif(window.top</name>
   <tag></tag>
   <elementGuidId>ca0be589-9a95-47db-a0ac-a4b1af6922d8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>form</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>name</name>
      <type>Main</type>
      <value>form1</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>post</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>action</name>
      <type>Main</type>
      <value>/ControllerServlet</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>target</name>
      <type>Main</type>
      <value>_self</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

























  






       
 
 var fndBtn =&quot;Y&quot;;

		
		if(window.top.document.forms[0].csHidden!=null &amp;&amp; window.top.document.forms[0].csHidden==&quot;Y&quot; &amp;&amp; fndBtn==&quot;Y&quot;){
			window.attachEvent( &quot;onload&quot;, findButtons );
		}
		




  
  
  
  







  
  
  



















var fw_nextTab=&quot;&quot;;
var fw_previousTab=&quot;tab1&quot;;
var fw_oldNextTab=&quot;&quot;;
var fw_oldPreviousTab=&quot;tab1&quot;;
var fw_LastTab=&quot;&quot;;
var fw_tabArray = new Array();
var fw_disableTabArray = &quot;&quot;;


function openModelessWindow(url,title,props){  
   return top.openHelpModal(url,title,props);
}
 


 





var myclose = false;
var submitHappening = false;

function confirmClose() {      
   var action = document.form1.ACTION.value;
   myclose=false;
   if (window.event.clientY &lt; 0 &amp;&amp; !submitHappening &amp;&amp; action != &quot;TIMEOUT&quot;) {           	
   	myclose=true;
   	var str = &quot;By Selecting OK, the system will log you out of VaCMS&quot;;           
	event.returnValue = str;	
    }           
} 


function handleOnClose() {       
      var action = document.form1.ACTION.value;     
      if (myclose || (window.event.clientX &lt; 0 &amp;&amp; window.event.clientY &lt; 0 &amp;&amp; 
      			!submitHappening &amp;&amp; action != &quot;TIMEOUT&quot;)) {                	        
          openCloseWindow();       
      }       
} 

function trimField(elem){
	trimSpaces(elem);
	return true;
}

function runReadOnlyApp() {
alert(&quot;1&quot;);
var rowin= window.open(&quot;loadexplore.html&quot;,&quot;test&quot;,&quot;&quot;);
}
window.onunload = handleOnClose;
//commented to fix the issue in IE version - dkandathil - 07/25
window.onbeforeunload = confirmClose;



function searchContext(){

	var url = &quot;&quot;;
	var type = document.form1.cntxtSearch.value;
	
	/* First letter as uppercase, rest lower */ 
	var str = document.form1.cntxtSearchText.value.trim();
	if(type == &quot;AP&quot;){
		var str = str.substring(0,1).toUpperCase() + str.substring(1,str.length); 
	}
	document.form1.cntxtSearchText.value = str;
	
	if((type == &quot;AP&quot;) &amp;&amp; isValidNumber('T',8,document.form1.cntxtSearchText,document.form1.cntxtSearchText)){
		url = &quot;/ControllerServlet?CS=Y&amp;appNum=&quot;;
		setPageAndAction('ARSAR','');
	}else if((type == &quot;CA&quot;) &amp;&amp; isNumeric(document.form1.cntxtSearchText)){
		url = &quot;/ControllerServlet?CS=Y&amp;caseOrApplicationNumber=&quot;;
		setPageAndAction('IQCSS','');
	}else if((type == &quot;PR&quot;) &amp;&amp; (trimField(document.form1.cntxtSearchText)) &amp;&amp; (isZero(document.form1.cntxtSearchText))){
		url = &quot;/ControllerServlet?selectedProvider=&quot;;
		setPageAndAction('PMPRD','initialize');
	}else if((type == &quot;IN&quot;) &amp;&amp; (trimField(document.form1.cntxtSearchText)) &amp;&amp; (isZero(document.form1.cntxtSearchText))){
		url = &quot;/ControllerServlet?cin=&quot;;
		setPageAndAction('IQISU','ClickOnLink');
	}else if((type == &quot;SS&quot;) &amp;&amp; validateSSN(document.form1.cntxtSearchText,'','')){
		url = &quot;/ControllerServlet?CS=Y&amp;ssnNumber=&quot;;
		setPageAndAction('IQISU','ClickOnLink');
	}else if((type == &quot;ED&quot;) &amp;&amp; (trimField(document.form1.cntxtSearchText)) &amp;&amp; (isZero(document.form1.cntxtSearchText))){
		url = &quot;/ControllerServlet?CS=Y&amp;selectedEdgValue=&quot;;
		setPageAndAction('IQESS','Search');
	}else{
		document.form1.cntxtSearchText.value=&quot;&quot;;
		document.form1.cntxtSearchText.focus();
		return false;
	}
	url = url + document.form1.cntxtSearchText.value;
	//code for back button in contextsearch and popup pages]
	//start
	var wfnav_hidden = document.form1.WF_NAV_ID;
	var wfstart_hidden = document.form1.WF_START;
	var wfnaviframe_hidden = document.form1.WF_NAV_RES_IFRAME;
	if(wfnav_hidden != null &amp;&amp; wfnav_hidden != &quot;undefined&quot;){
		document.form1.WF_NAV_ID.value = &quot;BACK_BUTTON&quot;;
		if(wfstart_hidden != null &amp;&amp; wfstart_hidden != &quot;undefined&quot;){
			document.form1.WF_START.value = &quot;Y&quot;;
		}else{
			url += &quot;&amp;WF_START=Y&quot;;
		}
	}else{
		url += &quot;&amp;WF_NAV_ID=BACK_BUTTON&amp;WF_START=Y&quot;;
	}
	if(wfnaviframe_hidden != null &amp;&amp; wfnaviframe_hidden != &quot;undefined&quot;){
		wfnaviframe_hidden.value = null;
	}
	//end
	document.form1.action = url;
	document.form1.method = &quot;POST&quot;;
	document.form1.submit();
	return true;
}

function setPageAndAction(pageID,actionID) {

// for displaying case,application and ssn
  if((pageID!=null) &amp;&amp; ((pageID==&quot;IQCSS&quot;)||(pageID==&quot;ARSAR&quot;))){
  	    document.form1.REQUESTED_PAGE_ID.value = pageID;
  	    document.form1.PAGE_ID.value = '';
  	    document.form1.ACTION.value=actionID;
  	    
// for displaying edg,provider and individual
  }else if((pageID!=null) &amp;&amp; ((pageID==&quot;IQESS&quot;)||(pageID==&quot;PMPRD&quot;)||(pageID==&quot;IQISU&quot;))){
    	document.form1.PAGE_ID.value = pageID;
    	document.form1.ACTION.value=actionID;
  }

  document.form1.target = '_self';
  
}

function setBriki(){
	var bUrl = &quot;&quot;;
    window.open(bUrl,'BRAD','left=1,top=1,width=950,height=700,toolbar=no, location=no,directories=no,status=yes,menubar=no,scrollbars=yes,resizable=yes');
}













   















  




  
    
    
    	
    		 Help
    	
	     Logout
		
     
  
 
 
  
   
   
   
   	
	
	Rishabh Jain
	
	
	
	
	USER ID: 
	
	
		
			xxx009

		
	
	
	
	Amherst
     
      
      
        
      
      Search By
         
      
	  
 
 APPLICATION
 CASE
 CLIENT
 

	  

	  
	   
	    
	  
      
      29 August 2018
   
	 
   
   
   
  
  
 
  
	   
		You are working in LIHEAP QAT environment 
	  
	  
 
 
	   
		Server Name :You are working in LIHEAP QAT environment 
	     Build Number :null 
	     Build Stream :null 
	     Build Time :null 
	  
	  
 
  

    
    
	




 
   
     
		
        
           
          
		   
					
						fw_tabArray[1]=79;
					
					
							
							
			
			Register
			
			
			
			
			
			
					
						fw_tabArray[2]=71;
					
					
				
			
			 
			
			Address	
			
		
						

			

			
			
			
			
			
		   
		  		    
			
	   	   
   
   


		
   fw_LastTab=&quot;tab2&quot;;
   
    if(tab1_Middle.style.display==&quot;none&quot;){
    	document.getElementById(&quot;previousButton&quot;).style.visibility=&quot;visible&quot;;	
    }
    
    if(eval(fw_LastTab+&quot;_Middle.style.display=='none'&quot;)){
    	document.getElementById(&quot;nextButton&quot;).style.visibility=&quot;visible&quot;;	
    }
    
   function previousTabFun(){
    	document.all.showAllTabs.style.display=&quot;none&quot;;	
		eval(fw_previousTab+&quot;_Middle.style.display='block'&quot;);
		eval(fw_previousTab+&quot;_RightTab.style.display='block'&quot;);
		var fw_width=0;
		for(var fw_iterate=1;fw_iterate&lt;(fw_tabArray.length);fw_iterate++){
			var fw_lenght = fw_tabArray[fw_iterate];
			if(eval(&quot;tab&quot;+(fw_iterate)+&quot;_Middle.style.display=='block'&quot;)){
				fw_width += parseInt(fw_lenght);
				if(fw_width >= 790){
					fw_nextTab=fw_oldNextTab;
					eval(fw_oldNextTab+&quot;_Middle.style.display='none'&quot;);
					eval(fw_oldNextTab+&quot;_RightTab.style.display='none'&quot;);
					fw_oldNextTab = fw_oldNextTab.split(&quot;tab&quot;);
					fw_oldNextTab[1]=parseInt(fw_oldNextTab[1])-1;
					fw_oldNextTab=&quot;tab&quot;+fw_oldNextTab[1];
					document.getElementById(&quot;nextButton&quot;).style.visibility=&quot;visible&quot;;
					fw_iterate=1;
					fw_width=0;
				}
			}
		}
		fw_oldPreviousTab = fw_previousTab;
		fw_previousTab = fw_previousTab.split(&quot;tab&quot;);
		if(!fw_oldPreviousTab.match(&quot;tab1&quot;)){
			fw_previousTab[1]=parseInt(fw_previousTab[1])-1;
			fw_previousTab=&quot;tab&quot;+fw_previousTab[1];
		}else{
			document.getElementById(&quot;previousButton&quot;).style.visibility=&quot;hidden&quot;;	
		}
		
		return false;
   }
		  
	function nextTabFun(){
		document.all.showAllTabs.style.display=&quot;none&quot;;	
		eval(fw_nextTab+&quot;_Middle.style.display='block'&quot;);
		eval(fw_nextTab+&quot;_RightTab.style.display='block'&quot;);
		
		var fw_width=0;
		for(var fw_iterate=1;fw_iterate&lt;(fw_tabArray.length);fw_iterate++){
			var fw_lenght = fw_tabArray[fw_iterate];
			if(eval(&quot;tab&quot;+(fw_iterate)+&quot;_Middle.style.display=='block'&quot;)){
				fw_width += parseInt(fw_lenght);
				if(fw_width >= 790){
					fw_previousTab=fw_oldPreviousTab;
					eval(fw_oldPreviousTab+&quot;_Middle.style.display='none'&quot;);
					eval(fw_oldPreviousTab+&quot;_RightTab.style.display='none'&quot;);
					fw_oldPreviousTab=fw_oldPreviousTab.split(&quot;tab&quot;);
					fw_oldPreviousTab[1]=parseInt(fw_oldPreviousTab[1])+1;
					fw_oldPreviousTab=&quot;tab&quot;+fw_oldPreviousTab[1];
					document.getElementById(&quot;previousButton&quot;).style.visibility=&quot;visible&quot;;
					fw_iterate=1;
					fw_width=0;
				}
			}
		}
		fw_oldNextTab = fw_nextTab;
		fw_nextTab=fw_nextTab.split(&quot;tab&quot;);
		if(fw_LastTab!=fw_oldNextTab){
			fw_nextTab[1]=parseInt(fw_nextTab[1])+1;
			fw_nextTab=&quot;tab&quot;+fw_nextTab[1];
		}else{
			document.getElementById(&quot;nextButton&quot;).style.visibility=&quot;hidden&quot;;	
		}
		
		return false;
	}

   
     
  
  






   
    
    
     Register Family
     
     
   			  
 

     
    
    
     
	
     
    
    	
    		





window.attachEvent(&quot;onload&quot;, function () {popupDivTagCheck();});
 function popupDivTagCheck(){
 		var flag =0;
		var popupDivImg = document.getElementById('userDetails');  
		if(document.getElementById('popupDivLayer1')){
			if(popupDivImg!=null){
				popupDivImg.style.display = 'inline';				
			}
			if(document.forms[0].USER_SHOW_STATE.value==&quot;Y&quot;){
				var obj = document.getElementById('popupDivLayer1');
				obj.style.visibility = 'visible';
			}

        } else {
        	if(popupDivImg!=null){
   	      	 popupDivImg.style.display = 'none';      		
        	}
		}
		return false;
 }

    	
    
        
    


 
     
 
     




   	
   	
     




	 	
   
 

 	
   	
 
    
 
 
  
  
  
  
  
    
  
   
  .select-free{	position:absolute;	z-index:1000;   overflow:hidden;	}.select-free iframe{display:none;display/**/:block;position:absolute;top:0;left:0;z-index:-1;filter:mask();width:3000px;height:3000px)}
   
          
    Session Ending Soon in   sec.
  This session is about to expire.
  
    
     
    
  &lt;h2>Blank Page&lt;/h2>
   
    
    

  
 
  if (window.addEventListener) {
	    // Check for addEventListener first, since IE9/10 have both
	    // but you should use the standard over the deprecated IE-specific one
	    window.addEventListener('load', runSessionTimer);
	    window.addEventListener('load', recordActive);
	} else if (window.attachEvent) {
	    window.attachEvent('onload', runSessionTimer);
	    window.attachEvent('onload', recordActive);
	} 
  

 














	
  
  
     
    
    














	
      		
        		 
		
	 
  

    
      
       
   			   
	  
	 
	



    
      
	  
	  	 
	  	    
	  
	 

 








    
      
	  
	  	 	

		 
	  	    
	  
	 


document.all.spaceTableClient.style.display='none';
document.all.errorTableClient.style.display='none';
document.all.messageTableClient.style.display='none';
document.all.bannerTableClientTop.style.display='none';
displayMessages(&quot;&amp;nbsp;&quot;,&quot;&quot;);







 
    
  

	
	






 	
 	
 

     
       
       Application/ Screening#:

      
        
        
      
		
			 
		
       
             
       
	   Status:

        
			
       
          
      
       
      
		
		
	   
	  Date Received:
 
        
      
      
  

	















function generateCorrespondence() {
	var corrElem = document.getElementById(&quot;corrSw&quot;);
	if(corrElem==null||corrElem.value==&quot;null&quot;||corrElem.value==&quot;undefined&quot;){
		corrElem.value=&quot;false&quot;;
	}
	var pageElem = document.getElementById(&quot;dcPageId&quot;);
	if(pageElem==null||pageElem.value==&quot;null&quot;||pageElem.value==&quot;&quot;){
			corrElem.value=&quot;false&quot;;
	}
	if(corrElem.value == &quot;true&quot; ) {	
		 openModalDialog(&quot;/ControllerServlet?ACTION=searchCorrespondence&amp;PAGE_ID=ARCOS&amp;cin=null&amp;dcPageId=null&quot;,'Manual Correspondence','dialogWidth:829px;dialogHeight:450px;help:no;scroll:yes;resizable:yes');
		resetCorrespondenceSw(corrElem);
	}
	return false;
}
		try{
			window.detachEvent(&quot;onload&quot;,generateCorrespondence);
		}catch(Exception){
		}

	


	    
	
	



  
 
        
 

 




    
  

			 




	
          

			

			Registration Information
			

          
        
		
          
			
			  
			
			
			
		
      
      Date Received: / / 

      
			
               Time Received:
	01
	02
	03
	04
	05
	06
	07
	08
	09
	10
	11
	12
	
	00
	01
	02
	03
	04
	05
	06
	07
	08
	09
	10
	11
	12
	13
	14
	15
	16
	17
	18
	19
	20
	21
	22
	23
	24
	25
	26
	27
	28
	29
	30
	31
	32
	33
	34
	35
	36
	37
	38
	39
	40
	41
	42
	43
	44
	45
	46
	47
	48
	49
	50
	51
	52
	53
	54
	55
	56
	57
	58
	59
	
	AM
	PM
	

      		
            

            
			
			
			
			
			
 			

          
             
              Primary Written Language:

      		
      		
            
 
 English
 Spanish
 Sign Language
 African
 Afrikaans
 Albanian
 Amharic
 Arabic
 Armenian
 Azerabaijani
 Bantu
 Bengali
 Berber
 Bisayan
 Bulgarian
 Burmese
 Cajun
 Cantonese
 Chamorro
 Cherokee
 Chinese
 Croatian
 Cushite
 Czech
 Dakota
 Danish
 Dutch
 Estonian
 Finnish
 Formosan
 French
 French Creole
 Fulani
 German
 Greek
 Gujarati
 Hebrew
 Hindi
 Hungarian
 Icelandic
 Ilocano
 Indonesian
 Irish Gaelic
 Italian
 Jamaican Creole
 Japanese
 Kannada
 Karachay
 Korean
 Krio
 Kru&amp; Ibo&amp; Yoruba
 Kurdish
 Laotian
 Lettish
 Lithuanian
 Macedonian
 Malagasy
 Malay
 Malayalam
 Mandarin
 Mande
 Marathi
 Mongolian
 Mon-Khmer&amp; Cambodian
 Navajo
 Nepali
 Norwegian
 Pampangan
 Panjabi
 Pashto
 Patois
 Pennsylvania Dutch
 Persian
 Polish
 Portuguese
 Romanian
 Russian
 Samoan
 Scottic Gaelic
 Sebuano
 Serbian
 Serbocroatian
 Sindhi
 Sinhalese
 Slovak
 Slovene
 Sudanic
 Swahili
 Swedish
 Syriac
 Tagalog
 Tamil
 Telugu
 Thai
 Tibetan
 Turkish
 Uighur
 Ukrainian
 Urdu
 Vietnamese
 Yiddish
 Somali
 Taiwanese
 Tigrinya
 Maltese
 Other
 Bosnian
 Haitian Creole
 

            
            

            
            
             
             
              
              
          
          
          
           
             
              Primary Spoken Language:

      		
            
 
 English
 Spanish
 Sign Language
 African
 Afrikaans
 Albanian
 Amharic
 Arabic
 Armenian
 Azerabaijani
 Bantu
 Bengali
 Berber
 Bisayan
 Bulgarian
 Burmese
 Cajun
 Cantonese
 Chamorro
 Cherokee
 Chinese
 Croatian
 Cushite
 Czech
 Dakota
 Danish
 Dutch
 Estonian
 Finnish
 Formosan
 French
 French Creole
 Fulani
 German
 Greek
 Gujarati
 Hebrew
 Hindi
 Hungarian
 Icelandic
 Ilocano
 Indonesian
 Irish Gaelic
 Italian
 Jamaican Creole
 Japanese
 Kannada
 Karachay
 Korean
 Krio
 Kru&amp; Ibo&amp; Yoruba
 Kurdish
 Laotian
 Lettish
 Lithuanian
 Macedonian
 Malagasy
 Malay
 Malayalam
 Mandarin
 Mande
 Marathi
 Mongolian
 Mon-Khmer&amp; Cambodian
 Navajo
 Nepali
 Norwegian
 Pampangan
 Panjabi
 Pashto
 Patois
 Pennsylvania Dutch
 Persian
 Polish
 Portuguese
 Romanian
 Russian
 Samoan
 Scottic Gaelic
 Sebuano
 Serbian
 Serbocroatian
 Sindhi
 Sinhalese
 Slovak
 Slovene
 Sudanic
 Swahili
 Swedish
 Syriac
 Tagalog
 Tamil
 Telugu
 Thai
 Tibetan
 Turkish
 Uighur
 Ukrainian
 Urdu
 Vietnamese
 Yiddish
 Somali
 Taiwanese
 Tigrinya
 Maltese
 Other
 Bosnian
 Haitian Creole
 

            
            
            
            
            
              
			
			
			  
			  
            
      		
            
            
          
          
        
              
              
              Source:

              
              
             
      		
      		
            
 
 
 Customer Portal
 Drop-off
 FFM
 Fax
 Mail
 Manual Conversion
 Paper Application
 Phone
 RDE
 Referral
 Walk-In
 

            
            
            
 			
      			Source Application Date (For Manual Conversion): / / 

      		
            

            
        

        




        
          
           Applicant
          
        
        
 		
			
			
		
		
			
			
			
			Prefix:
	Dr.
	Miss
	Mr.
	Mrs.
	Ms.
	 First Name:Middle Name: Last Name:Suffix:
	I
	II
	III
	IV
	Jr
	Sr
	V
	VI
	

			
		
	
        

        
          
             Contact Information
          
        
		
          
            
			Home #:   

            
            
			Work #:  x  

            
            
			Cell/Other #:   

            
          
                    
          
          
             Correspondence Information
          
        
        
          
          	
			Preferred Correspondence Cell #:   

						
			
              Preferred Correspondence Email:

        	
            
              

            
          
   		                       
             
              Preferred Method of Correspondence:

               
 			
            
            
 
 Cell
 Email
 U.S. Mail
 

            
           
              Preferred Service Provider:
              
 			
            
            
 
 
 AT&amp;T Enterprise Paging
 AT&amp;T Global Smart Messaging Suite
 AT&amp;T Mobility
 AT&amp;T Mobility (formerly Cingular)
 AT&amp;T grandfathered customers
 Aio Wireless
 Airfire Mobile
 Alaska Communications
 Alltel (Allied Wireless)
 Ameritech
 Assurance Wireless
 BellSouth
 Bluegrass Cellular
 Bluesky Communications
 Boost Mobile
 C Beyond (All Page Wireless)
 C Spire Wireless
 Cellcom
 Cellular South
 Chariton Valley Wireless
 Chat Mobility
 Cincinnati Bell
 Cingular (Postpaid)
 Cleartalk
 Cricket
 DTC
 Edge Wireless
 Element Mobile
 Esendex
 General Communications Inc.
 Golden State Cellular
 Greatcall
 Hawaiian Telcom Wireless
 Kajeet
 LongLines
 MetroPCS
 Nextech
 Page Plus Cellular(Verizon MVNO)
 Pioneer Cellular
 Pocket Wireless
 Qwest Wireless
 Red Pocket Mobile(ATnull MVNO)
 Rogers Wireless
 Simple Mobile
 South Central Communications
 Southernlinc
 Sprint
 Straight Talk(ATT)
 Straight Talk(SprintPCS)
 Straight Talk(Verizon)
 Syringa Wireless
 T-Mobile
 Teleflip
 Ting
 TracFone (prepaid)
 US Cellular
 USA Mobility
 Unicel
 Union Wireless
 Verizon Wireless
 Viaero
 Virgin Mobile
 Voyager Mobile
 West Central Wireless
 XIT Communications
 i wireless (T-Mobile)
 i-wireless (Sprint PCS)
 nTelos
 

            
            
          
        
        
        Programs
			
			
			 	Child Care 
			 	Medical Assistance 
			 	Medical Assistance with Resource Assessment
			 	Resource Assessment Only
			
			
				SNAP
				TANF
				  
				
			
			
				
			
			
		
		
		 
		
		
		Is this a VaCAP SNAP Application?
            
         
        
 
 
 NO
 YES
 
 
        First day of first continuous period of institutionalization date:

        
            
              / / 
 
        
		
		 
		
		
		
        
        
          
		 
		  	



       
        







    
  













            
            
			
					
			
			

 			






      
    
  
  

      
 






var tabKey;
var win;
var timer;
var childOpen = true;

function getURL(selection) {
	 var str = &quot;&quot; 
	    var currTime = new Date().getTime();
	   if (selection == 'N') {
	     str = 'dynamicincludes/LeftNavigation.jsp?avoidCache='+currTime;
	   } else if (selection == 'O') {  
	 	    str = 'jsp/se/SEORGorganizer.jsp?avoidCache='+currTime;
	   } else if (selection == 'C') {   
	      str = 'jsp/se/SECSICaseInfo.jsp?avoidCache='+currTime;
	   } else if (selection == 'H') {   
	     str = 'jsp/se/SEHistory.jsp?avoidCache='+currTime;
	   }
	   return str;
}



function changeTab(key) {  
  var hisTab = document.getElementById('historyTabImg');
  var caseInfoTab = document.getElementById('caseInfoTabImg');
  var navTab = document.getElementById('navigationTabImg');
  var orgTab = document.getElementById('organizerTabImg');

  if (key == 'N') {
    navTab.src = &quot;images/tabNavigationSelected.jpg&quot;;
    if (caseInfoTab.src.indexOf(&quot;images/tabCaseInfoSelected.jpg&quot;)>0) caseInfoTab.src = &quot;images/tabCaseInfo.jpg&quot;;
    if (hisTab.src.indexOf(&quot;images/tabHistorySelected.jpg&quot;)>0) hisTab.src = &quot;images/tabHistory.jpg&quot;;
    if (orgTab.src.indexOf(&quot;images/tabOrganizerSelected.jpg&quot;)>0) orgTab.src = &quot;images/tabOrganizer.jpg&quot;;
  } else if (key == 'O') {
    if (navTab.src.indexOf(&quot;images/tabNavigationSelected.jpg&quot;)>0) navTab.src = &quot;images/tabNavigation.jpg&quot;;
    if (caseInfoTab.src.indexOf(&quot;images/tabCaseInfoSelected.jpg&quot;)>0) caseInfoTab.src = &quot;images/tabCaseInfo.jpg&quot;;
    if (hisTab.src.indexOf(&quot;images/tabHistorySelected.jpg&quot;)>0) hisTab.src = &quot;images/tabHistory.jpg&quot;;
    orgTab.src = &quot;images/tabOrganizerSelected.jpg&quot;;
  } else if (key == 'C') {
    if (navTab.src.indexOf(&quot;images/tabNavigationSelected.jpg&quot;)>0) navTab.src = &quot;images/tabNavigation.jpg&quot;;
    caseInfoTab.src = &quot;images/tabCaseInfoSelected.jpg&quot;;
    if (hisTab.src.indexOf(&quot;images/tabHistorySelected.jpg&quot;)>0) hisTab.src = &quot;images/tabHistory.jpg&quot;;
    if (orgTab.src.indexOf(&quot;images/tabOrganizerSelected.jpg&quot;)>0) orgTab.src = &quot;images/tabOrganizer.jpg&quot;;
  } else if (key == 'H') {
    if (navTab.src.indexOf(&quot;images/tabNavigationSelected.jpg&quot;)>0) navTab.src = &quot;images/tabNavigation.jpg&quot;;    
    if (caseInfoTab.src.indexOf(&quot;images/tabCaseInfoSelected.jpg&quot;)>0) caseInfoTab.src = &quot;images/tabCaseInfo.jpg&quot;;
    hisTab.src = &quot;images/tabHistorySelected.jpg&quot;;
    if (orgTab.src.indexOf(&quot;images/tabOrganizerSelected.jpg&quot;)>0) orgTab.src = &quot;images/tabOrganizer.jpg&quot;;
  }

}

function startCalendar() { 
	/*if(document.form1.dateReq!=null){
		var dateValue = document.form1.dateReq.value;
		dateValue=dateValue.split(&quot;/&quot;);
		document.form1.month.value=dateValue[0];
		document.form1.day.value=dateValue[1];
		document.form1.year.value=dateValue[2];
	}
	pickDate(document.form1.month,document.form1.day,document.form1.year,'dimg_cal','FW_OrgCalFrame');//,document.all.FW_OrgCalFrame,window.frames.FW_OrgCalFrame);
	*/
}

function ToggleDisplay() 
{
	img1 = document.getElementById(&quot;parent1&quot;);
	
	if ((child1.style.display == &quot;&quot;) || (child1.style.display == &quot;none&quot;)) {
		child1.style.display = &quot;block&quot;;
		img1.src = &quot;/images/leftNavMinus.gif&quot;;
		startCalendar();
	}	
	else 
	{
		child1.style.display = &quot;none&quot;;
		img1.src = &quot;/images/leftNavPlus.gif&quot;;
		startCalendar();
	}
}
function deleteRow(r,pageId)
{
  var i=r.parentNode.parentNode.rowIndex;
  document.getElementById('child1').deleteRow(i);
  deleteFavorite(pageId);
}

function deleteFavorite(pageId) {
	var childOpen = true;
	win = window.open(&quot;/ControllerServlet?REQUESTED_PAGE_ID=FWFAV&amp;FWPOPUP=Y&amp;REQUESTED_ACTION=deleteFavorite&amp;FAV_PAGE_ID=&quot; + pageId,null,
    &quot;height=1px,width=1px,status=no,toolbar=no,menubar=no,location=no&quot;);
    win.blur();
}

function makeAJAXRequest(key) {
	  
       var url = getURL(key);	
	 var http_request = false; 
	if (window.XMLHttpRequest) { // Mozilla, Safari, ... 
		http_request = new XMLHttpRequest(); 
		if (http_request.overrideMimeType) { 
			http_request.overrideMimeType('text/xml'); // See note below about this line 
		} 
	} else if (window.ActiveXObject) { // IE 
		try { 
		  http_request = new ActiveXObject(&quot;Msxml2.XMLHTTP&quot;); 
		} catch (e) {
		   try { 
			http_request = new ActiveXObject(&quot;Microsoft.XMLHTTP&quot;); 
		   } catch (e) {} 
		} 
	}
	 if (!http_request) { 
		alert('Giving up :( Cannot create an XMLHTTP instance'); 
		return false; 
	}
	 http_request.onreadystatechange = function() { reactToReponse(http_request,key); };
	 http_request.open('GET', url, false);
	 http_request.send(null); 
  	
	 return false;
} 

function reactToReponse(http_request,key) {
	tabKey=key;
  	var wFrame=window.frames.FW_OrgCalFrame;
    var divTag = document.getElementById(&quot;BRGLEFTAREA&quot;);
     var calObj = document.all.FW_CalFrame;
     var orgcalObj = document.all.FW_OrgCalFrame;
     var startHtml = &quot;&lt;table id='leftItems' title='Left Navigation' summary='Left Navigation' cellspacing=0 cellpadding=0 border='0' style='height:430px;width=157px' ;'>&quot;;
     startHtml = startHtml + &quot; &lt;tr valign='top'>&lt;td> &quot;;
     var endHtml = &quot; &lt;/td>&lt;/tr>&lt;/table>&quot;
	if (http_request.readyState == 4) { 
  	  if (http_request.status == 200) { 
	      var newHtml = http_request.responseText; 		
		var len = divTag.childNodes.length;		
         for (var i=0;i&lt;len;i++) {
         	if(divTag.childNodes[i]!=null){
			 	divTag.removeChild(divTag.childNodes[i]);
         	}
         }	
		if (key == 'N') {		   			
		     eval(newHtml);     		   		  
		} else if (key != 'N') {
		   newHtml = startHtml+newHtml+endHtml;       
		   divTag.innerHTML = &quot;&quot;;
		   divTag.innerHTML = newHtml;		
		} 
		if (calObj != null) {
	       document.all.FW_CalFrame.style.display=&quot;none&quot;;
		} 
		if (orgcalObj != null ) {
			if(orgcalObj.style){
				orgcalObj.style.display=&quot;none&quot;;
			}
		} 
	    changeTab(key);
	    if (key == 'O') {  
	    	startCalendar();
        } 
	      	   
	   } else { alert('There was a problem with the request.'); }
      }
}





    
   
     
     
    

    
  
    
   
    

    
  
    
     
  
 
  
  	
		
			
			
			
			
				
					
VaCMS Home Dashboard Tools AnnouncementsSearch   AnnouncementsMaintain   Announcements Tasks/RemindersEmployee   Tasks/Reminders  Create Manual   Tasks/Reminders   SchedulingSchedule   AppointmentSearch for   AppointmentsView My   CalendarView Unit   CalendarSchedule Unit   AppointmentWorker   Availability Rapid Data Entry   (RDE)Search   ApplicationEnter ApplicationSearch CaseSearch Fast Track   Client Application/ScreeningRegister an   Application/   Screening Maintain   Application/   ScreeningSearch   Application/   ScreeningApplication/   Screening   Disposition My WorkspaceSSA Referral   InboxLocality Inbox Inbox ReportsGenerate   ReportView Reports EAPEAP InboxEAP Application   DetailsEAP Client   DetailsWorker AssignmentEAP Client   Additional   InformationFuel AssistanceCrisis AssistanceCooling   AssistanceEAP Income   DetailsAuthorized   RepresentativeEAP Case Name   ChangeEAP Program   DenialEAP ReinstateEAP Case   Comments Data CollectionCase ActionCase Comments Individual   InformationInitiate   ReviewInitiate   ReinstateHouseholdHousehold   AddressClientConsentProgram   RequestExpedited   SNAPVoter   RegistrationInterviewSNAP/TANF Case   Level DetailsTANF   Diversionary   AssistanceTANF Emergency   AssistanceChild Care   ResourcesRelationshipEducationLiving   ArrangementClient   DemographicsAlienCustodial   PartyAbsent ParentTax ReturnQuestionsOther In State   BenefitsPregnancyDisabilityFoster CarePursuit of   BenefitsMA   Institutionaliz  ed IndividualMedicaid   Covered GroupBoarderFelony Drug   Conviction Non FinancialQuestionsImmunizationDCSE ReferralThird Party   ResourcesEmployment   Health   InsurancePast Health   InsuranceSNAP Work   RequirementsSNAP Work   Requirement   ClockTANF Work   Requirements60 Month   Federal Clock24-Month VIEW   Clock IncomeQuestionsEmploymentEmployment   BudgetSelf   EmploymentSelf-   Employment   Income BudgetUnearned   IncomeUnearned   Income BudgetOther   DeductionsYearly Income ResourcesQuestionsLiquid AssetAsset   Verification   SearchReal PropertyLife InsuranceBurial   ResourceVehiclePersonal   PropertyResource   TransferLTC Resource   AssessmentHardship ExpensesQuestionsDependent CareMedical   ExpensesMedicare   PremiumChild Support   and AlimonyHousing/Shelter   ExpensesUtility   ExpensesWork Related   ExpensesSpenddown   Medical   ExpenseSNAP Expenses   Standards Sanctions / IPVQuestionsNon-  ComplianceIPV   Disqualificatio  n Service Plans and   AssessmentChild Care   Service PlanHousehold   Needs   Assessment WaitlistWaitlist   HouseholdWaitlist   PeopleWaitlist   Relationships Wrap UpAlternate   PayeeAuthorized   counselorManual   Conversion MiscellaneousProgram DenialVerification   ChecklistExtended   Medical   Earning Status EligibilityRun EligibilityEligibility   SearchLTC Resource   AssessmentEligibility   SummaryRetroactive   Patient Pay   AdjustmentCertification   Period OverrideTANF Method of   IssuanceSNAP Card   Issuance   InformationMMIS Individual   DetailsMMIS Transaction   HistoryMatch MMIS   Case/Enrollee IDs   in VaCMSResource   Assessment   OverrideFFM Referral Benefit IssuanceBenefit DetailsBenefit   AdjustmentSNAP Mass   Benefits   ReplacementMaintain ChecksTANF DA/EA ClaimsView/Track   ClaimsTOP ClaimsPost-Unpost   ClaimPaymentsTOP Address   Request   ResponseTOP   Collections/Rev  ersalsTOP Claim   StatusTOP UpdatesTOP   Unprocessable   Records Time LimitsTANF ClocksSNAP Work   Requirement EAPEAP Credit   AuthorizationEAP Fuel   IssuanceEAP SDOPEAP Billing   InformationEAP FundingEAP Matrix   CalculatorEAP Security   Deposit/   Warranty ESPESP Queue ESP EnrollmentEnroll/UpdateStatus UpdateAssessmentActivity   DetailSupportive /   Transitional   ServiceNon ComplianceHardship   ExceptionReassign ESP   WorkerCreate Referral Foster Care DCSE   ReferralCreate New   ReferralEdit Pending   ReferralClose Active   Referral FormsView Pending   FormsView Forms   HistoryGenerate Manual RedeterminationPacket Received   DateMA Renewal   Manager InterfacesPARISAnnual IncomeChild Support   Enforcement   CollectionsDCSE Prior Month   Grant ReferraleDRS SNAP   DisqualificationNew Hire   Information Transaction LogsView Transaction   Log Security MaintenanceMaintain   BusinessSearch RolesAdd Roles Access ControlMap Business   To PagesMap Role to   BusinessMap Role to   Reports Identity ProofingAuthenticate   User-With   PINAuthenticate   User-Without   PINAnnouncements InquiryApplicationAddressCaseCase Load SearchClientElectronic   Sources Request   History Multi ProgramPending   ApplicationRenewal StatusInterim Report EAPLocality   ExpendituresApproved   VendorsPending by   ComponentPayment   HistoryAllocation   StatusCrisis or   Cooling Funds   Projection Link ClientLink Client Replacement CardsMA Replacement   CardSNAP Replacement   Card DMISDocument SearchDocument UploadDocument   SeparatorDocument InboxDocument- Forms   Search Authorization Management Maintain   AuthorizationAuthorization   SearchStatus InfoGeneral   InformationFunding   ProgramCo-Pay   SummaryNeed for   Services   School   ScheduleStandard   ScheduleSchedule   Calendar School ScheduleSchool   Schedule   SearchAuthorization   Rollover Funds ManagementFunding AgencyBudget LineFunding   AllocationGrantsBudget Line Over   Encumbered AmountAvailable FundsWaitlist   ManagementExpenditure   SummaryExpenditure   Exceptions Maintain LDSS   Resources Build ProfileLDSSUnitEmployee Search/Maintain   ProfileLDSSUnitEmployeeMap Hospitals to   FIPSCase /   Application   AssignmentWorkload   Reassignment Reference TablesSearch / View   Reference TableSearch Reference   Table Version Maintain   Reference TablesAdd / Update   Reference   TableReference   Table FieldsReference   Table Fields   (nv)Reference   Table ValuesReference   Table Values   (nv) Administer   Reference TableMaintain Field   Domain Vendor ManagementEnroll Vendor Vendor SearchMaintain   VendorFacility   AddressFacility   DemographicFacility   StatusVendor Payment   InfoVendor ServiceFacility RatesFacility   ContactFacility   HouseholdFacility   Narrative Legal Entity   SearchLegal Entity   SummaryMaintain Legal   EntityLegal Entity   Address DetailMaximum   Reimbursable   RatesVendor   Withholdings Ad Hoc Reports Child CareAverage Cost   Per FamilyChild Care   DemographicsClient Mailing   LabelsCollectionsChildren Per   VendorVendor Mailing   LabelsVendor   Requirements   DueVendor Service   DetailWait List -   Action TakenFEE Cases by   OfficeCaseload by   WorkerVendor   DemographicsStatewide   Vendor   Directory   ReportWait List   Details ReportSFY   Authorizations   Report1099 ReportStatewide   Allocation   Summary Report Multi ProgramCase Alpha   Listing VDSS AppealsHearing   Disposition   and IssuesAppeals   Processing Fraud ManagementADH   OUTCOMESCourt   DispositionFraud   Allocation   Claims-  OverpaymentsFraud   Management   Overpayment SNAP366-B   SNAP/DSNAPUsers Role   Assignment Reports Search Child CareCase   ManagementFinancial   ManagementVendor   ManagementMedicaidDMISAsset   VerificationMulti ProgramEAPFinanceFraud ManagementSNAPVDSS AppealsTANF SNAP ApptrackAPPTRACK UpdateCorrective Action   Inquiry/Add ConversionConversion Client   Cross ReferenceConversion Case   Cross ReferenceMMIS Automated   Conversion Case   Search PaymentsAttendance SearchAdjustment SearchVendor Payment   SearchReceivables   Account SearchPayment Search -   CaseVendor Payment   Summary FraudFraud InquiryCreate Referral ReportsInvestigation   CompletedOverpayment   DetailCourt   DispositionPending in   Court SystemOverdue   InvestigationsMonthly ReportADH Outcomes ReferralReferral Agency -   InquiryReferral InquiryReferral Details Useful LinksUseful Links
				
				   
			    SPIDeR
				
			
				
		
     
  

 








	window.attachEvent(&quot;onfocus&quot;,setFocusOnFirstField);
	
      




</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/form[1]</value>
   </webElementProperties>
</WebElementEntity>
