import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://vacms.enh.sit.dss.virginia.gov/SELoginAccess.jsp?fromIndex=true')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_userId'), 'xxx009')

WebUI.setEncryptedText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_password'), '0EE/MRa7Qf83C3pvC4k1lA==')

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/a_Disclaimer'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/a_ApplicationScreening'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_monthdateReceived'), '07')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_datedateReceived'), '13')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_yeardateReceived'), '2018')

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td_Medical Assistance'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ltc'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_firstName'), 'jon')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_lastName'), 'Flo')

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_streetNumber'), '23')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_streetName2'), 'Main')

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_AlleyAnnexArcadeAvenueB'), 
    'SH', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_AlleyAnnexArcadeAvenueB'), 
    'SN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_AlleyAnnexArcadeAvenueB'), 
    'SV', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_AlleyAnnexArcadeAvenueB'), 
    'SM', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_AlleyAnnexArcadeAvenueB'), 
    'ST', true)

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_city'), 'Richmond')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_zipCode5'), '23219')

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Client statement Collat'), 
    'LS', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_getLocality'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionButton6'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionButton6'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_1'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionButton6'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Female Male'), 
    'M', true)

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn1ssn'), '555')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn2ssn'), '55')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn3ssn'), '0816')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_monthdateOfBirth'), '07')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_datedateOfBirth'), '02')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_yeardateOfBirth'), '1975')

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_American Indian or Alas'), 
    'WH', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Chicanoa Cuban Hispanic'), 
    'UK', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionButton3'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_firstName'), 'Jane')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_lastName'), 'Flo')

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Female Male'), 
    'F', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td_-  -'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn1ssn'), '444')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn2ssn'), '44')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_ssn3ssn'), '0816')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_monthdateOfBirth'), '08')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_datedateOfBirth'), '16')

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_yeardateOfBirth'), '1983')

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_American Indian or Alas'), 
    'WH', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Chicanoa Cuban Hispanic'), 
    'UK', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_2'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_hoh'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_hoh_1'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_3'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_3'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_aidRequestSw_Ma'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionButtonNext'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_primaryWorkerId'), 'xxx009')

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_button4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/a_Data Collection'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/a_Case Action'))

WebUI.setText(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_caseOrApplicationNumber'), 
    'T13055576')

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td_IntakeScreening Case Change'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_IntakeScreening Case Ch'), 
    'PI', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_SUBMIT (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_1'), 
    'Y', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Baptismal records Drive'), 
    'PP', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Social Security Card Sy'), 
    'SC', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_In Household Out of Hou'), 
    'Y', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Baptismal records Drive'), 
    'PP', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Social Security Card Sy'), 
    'SC', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_In Household Out of Hou'), 
    'Y', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_YES NO Yes with no SSN'), 
    'Y', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_2'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_FatherSonBrother Half-B'), 
    'HU', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Adult Care ResidenceAdu'), 
    'PR', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Adult Care ResidenceAdu'), 
    'PR', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_3'), 
    'Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Client statement Collat_1'), 
    'LS', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Exempt NO YES'), 
    'Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Adult-Not Applicable Am'), 
    'US', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Non-filer  Tax dependen'), 
    'TP', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_4'), 
    'N', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_5'), 
    'N', true)

WebUI.doubleClick(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_6'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_3'), 
    'Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Client statement Collat_1'), 
    'LS', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Exempt NO YES'), 
    'Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Adult-Not Applicable Am'), 
    'US', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Non-filer  Tax dependen'), 
    'TX', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_4'), 
    'N', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_5'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_6'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Flo Jon 43MFlo Jane 35F'), 
    '2103234268', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Flo Jane 35F'), 
    '2103234271', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_Flo Jon 43M'), 
    '2103234268', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_SUBMIT_1 (1)'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_next'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_submit_4'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_SUBMIT (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/select_NO YES_7'), 
    'N', true)

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_SUBMIT_1 (1)'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_next'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_MMISCaseId_1'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_MMISCaseId_2'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionbutton1'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/img_editIndv'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_MMISEnrolleeId'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_button2'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/img_editIndv'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_MMISEnrolleeId'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/img_editIndv'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_MMISEnrolleeId'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_button2'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_actionbutton2'))

WebUI.rightClick(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td_'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/table_90384872  MA-Plan First'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/table_90384872  MA-Plan First'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td_No'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756382'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756384'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756383'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756385'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756386'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_dispose_2756388'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/input_certifyAuthButton'))

WebUI.click(findTestObject('Object Repository/RegisterClient/remaningDataEntry/Page_VaCMS/td__1'))

